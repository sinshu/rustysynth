#![allow(dead_code)]

use crate::loop_mode::LoopMode;
use crate::synthesizer_settings::SynthesizerSettings;

// In this class, fixed-point numbers are used for speed-up.
// A fixed-point number is expressed by Int64, whose lower 24 bits represent the fraction part,
// and the rest represent the integer part.
// For clarity, fixed-point number variables have a suffix "_fp".
//
// `fill_block_no_loop` and `fill_block_continuous` first try a fast path, which checks once per block that no sample reaches the end
// (or wraps the loop), and then fills the block with `fill_block_in_bounds`. Otherwise they fall back to the `_checked` versions,
// which test the end or the loop point for every sample.

#[derive(Debug)]
#[non_exhaustive]
pub(crate) struct Oscillator {
    synthesizer_sample_rate: i32,

    loop_mode: LoopMode,
    sample_sample_rate: i32,
    start: i32,
    end: i32,
    start_loop: i32,
    end_loop: i32,
    root_key: i32,

    tune: f32,
    pitch_change_scale: f32,
    sample_rate_ratio: f32,

    looping: bool,

    position_fp: i64,

    // Cache: the pitch change last seen and the ratio computed from it. Most notes hold one pitch, and `powf` is expensive.
    last_pitch_change: f32,
    last_pitch_ratio: f32,
}

impl Oscillator {
    const FRAC_BITS: i32 = 24;
    const FRAC_UNIT: i64 = 1_i64 << Oscillator::FRAC_BITS;
    const FP_TO_SAMPLE: f32 = 1_f32 / (32768 * Oscillator::FRAC_UNIT) as f32;

    pub(crate) fn new(settings: &SynthesizerSettings) -> Self {
        Self {
            synthesizer_sample_rate: settings.sample_rate,
            loop_mode: LoopMode::NoLoop,
            sample_sample_rate: 0,
            start: 0,
            end: 0,
            start_loop: 0,
            end_loop: 0,
            root_key: 0,
            tune: 0_f32,
            pitch_change_scale: 0_f32,
            sample_rate_ratio: 0_f32,
            looping: false,
            position_fp: 0,
            last_pitch_change: f32::NAN,
            last_pitch_ratio: 0_f32,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn start(
        &mut self,
        loop_mode: LoopMode,
        sample_rate: i32,
        start: i32,
        end: i32,
        start_loop: i32,
        end_loop: i32,
        root_key: i32,
        coarse_tune: i32,
        fine_tune: i32,
        scale_tuning: i32,
    ) {
        self.loop_mode = loop_mode;
        self.sample_sample_rate = sample_rate;
        self.start = start;
        self.end = end;
        self.start_loop = start_loop;
        self.end_loop = end_loop;
        self.root_key = root_key;

        self.tune = coarse_tune as f32 + 0.01_f32 * fine_tune as f32;
        self.pitch_change_scale = 0.01_f32 * scale_tuning as f32;
        self.sample_rate_ratio = sample_rate as f32 / self.synthesizer_sample_rate as f32;
        // Looping is disabled when the loop range is invalid (start >= end),
        // so the region plays once instead of reading past the loop (#55).
        self.looping = self.loop_mode != LoopMode::NoLoop && self.start_loop < self.end_loop;
        self.position_fp = (start as i64) << Oscillator::FRAC_BITS;
        self.last_pitch_change = f32::NAN;
    }

    pub(crate) fn release(&mut self) {
        if self.loop_mode == LoopMode::LoopUntilNoteOff {
            self.looping = false;
        }
    }

    pub(crate) fn process(&mut self, data: &[i16], block: &mut [f32], pitch: f32) -> bool {
        let pitch_change = self.pitch_change_scale * (pitch - self.root_key as f32) + self.tune;
        if pitch_change != self.last_pitch_change {
            self.last_pitch_ratio = self.sample_rate_ratio * 2_f32.powf(pitch_change / 12_f32);
            self.last_pitch_change = pitch_change;
        }
        self.fill_block(data, block, self.last_pitch_ratio as f64)
    }

    fn fill_block(&mut self, data: &[i16], block: &mut [f32], pitch_ratio: f64) -> bool {
        let pitch_ratio_fp = (Oscillator::FRAC_UNIT as f64 * pitch_ratio) as i64;

        if self.looping {
            self.fill_block_continuous(data, block, pitch_ratio_fp)
        } else {
            self.fill_block_no_loop(data, block, pitch_ratio_fp)
        }
    }

    fn fill_block_no_loop(&mut self, data: &[i16], block: &mut [f32], pitch_ratio_fp: i64) -> bool {
        // Fast path: if the whole block lies within the sample (the usual case), the end is not tested for every sample.
        if !block.is_empty() && pitch_ratio_fp >= 0 {
            let last_position_fp = self.position_fp + (block.len() as i64 - 1) * pitch_ratio_fp;
            let last = (last_position_fp >> Oscillator::FRAC_BITS) as usize;
            if last < self.end as usize {
                self.fill_block_in_bounds(data, block, pitch_ratio_fp, last);
                return true;
            }
        }

        self.fill_block_no_loop_checked(data, block, pitch_ratio_fp)
    }

    fn fill_block_no_loop_checked(
        &mut self,
        data: &[i16],
        block: &mut [f32],
        pitch_ratio_fp: i64,
    ) -> bool {
        for t in 0..block.len() {
            let index = (self.position_fp >> Oscillator::FRAC_BITS) as usize;
            if index >= self.end as usize {
                if t > 0 {
                    let len = block.len();
                    block[t..len].fill(0_f32);
                    return true;
                } else {
                    return false;
                }
            }

            let x1 = data[index] as i64;
            let x2 = data[index + 1] as i64;
            let a_fp = self.position_fp & (Oscillator::FRAC_UNIT - 1);
            block[t] = Oscillator::FP_TO_SAMPLE
                * ((x1 << Oscillator::FRAC_BITS) + a_fp * (x2 - x1)) as f32;

            self.position_fp += pitch_ratio_fp;
        }

        true
    }

    fn fill_block_continuous(
        &mut self,
        data: &[i16],
        block: &mut [f32],
        pitch_ratio_fp: i64,
    ) -> bool {
        // Fast path: if the block stays within the loop (the usual case), the wrap is not tested for every sample.
        // The checked code wraps the position at the first sample. Here that wrap is applied to a local copy
        // so that both paths agree, and it is committed only if the fast path is taken.
        let end_loop_fp = (self.end_loop as i64) << Oscillator::FRAC_BITS;
        let loop_length_fp = ((self.end_loop - self.start_loop) as i64) << Oscillator::FRAC_BITS;
        let mut position_fp = self.position_fp;
        if position_fp >= end_loop_fp {
            position_fp -= loop_length_fp;
        }
        if !block.is_empty() && pitch_ratio_fp >= 0 {
            let last_position_fp = position_fp + (block.len() as i64 - 1) * pitch_ratio_fp;
            let last = (last_position_fp >> Oscillator::FRAC_BITS) as usize;
            if last_position_fp < end_loop_fp && last + 1 < self.end_loop as usize {
                self.position_fp = position_fp;
                self.fill_block_in_bounds(data, block, pitch_ratio_fp, last);
                return true;
            }
        }

        self.fill_block_continuous_checked(data, block, pitch_ratio_fp)
    }

    fn fill_block_continuous_checked(
        &mut self,
        data: &[i16],
        block: &mut [f32],
        pitch_ratio_fp: i64,
    ) -> bool {
        let end_loop_fp = (self.end_loop as i64) << Oscillator::FRAC_BITS;
        let loop_length = (self.end_loop - self.start_loop) as i64;
        let loop_length_fp = loop_length << Oscillator::FRAC_BITS;

        for sample in block.iter_mut() {
            if self.position_fp >= end_loop_fp {
                self.position_fp -= loop_length_fp;
            }

            let index1 = (self.position_fp >> Oscillator::FRAC_BITS) as usize;
            let mut index2 = index1 + 1;
            if index2 >= self.end_loop as usize {
                index2 -= loop_length as usize;
            }

            let x1 = data[index1] as i64;
            let x2 = data[index2] as i64;
            let a_fp = self.position_fp & (Oscillator::FRAC_UNIT - 1);
            *sample = Oscillator::FP_TO_SAMPLE
                * ((x1 << Oscillator::FRAC_BITS) + a_fp * (x2 - x1)) as f32;

            self.position_fp += pitch_ratio_fp;
        }

        true
    }

    /// Fills a block that neither wraps nor reaches the end. The data is read through a single slice covering every index the block touches
    /// (the first index through `last + 1`), where `last` is the index of the block's final sample. The values are those of the checked code.
    fn fill_block_in_bounds(
        &mut self,
        data: &[i16],
        block: &mut [f32],
        pitch_ratio_fp: i64,
        last: usize,
    ) {
        let first = (self.position_fp >> Oscillator::FRAC_BITS) as usize;
        let window = &data[first..=last + 1];
        for sample in block.iter_mut() {
            let index = (self.position_fp >> Oscillator::FRAC_BITS) as usize - first;
            let x1 = window[index] as i64;
            let x2 = window[index + 1] as i64;
            let a_fp = self.position_fp & (Oscillator::FRAC_UNIT - 1);
            *sample = Oscillator::FP_TO_SAMPLE
                * ((x1 << Oscillator::FRAC_BITS) + a_fp * (x2 - x1)) as f32;

            self.position_fp += pitch_ratio_fp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // a small generator, so that the tests need no dependency
    fn next(state: &mut u64) -> u64 {
        *state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        *state >> 33
    }

    fn oscillator(
        looping: bool,
        end: i32,
        start_loop: i32,
        end_loop: i32,
        position: i64,
    ) -> Oscillator {
        let mut o = Oscillator::new(&SynthesizerSettings::new(44100));
        o.start = 0;
        o.end = end;
        o.start_loop = start_loop;
        o.end_loop = end_loop;
        o.looping = looping;
        o.position_fp = position << Oscillator::FRAC_BITS;
        o
    }

    // The fast path must give the same samples and the same position as the checked code, whatever the block does:
    // stays within the sample or loop, wraps the loop, reaches the end, or starts past it;
    // with ratios from nearly still to several octaves up, and blocks of several lengths.
    #[test]
    fn test_fast_path_matches_checked() {
        let mut state = 1_u64;
        let data: Vec<i16> = (0..2000)
            .map(|_| (next(&mut state) % 65536) as i64 as i16)
            .collect();
        for _ in 0..4000 {
            let looping = next(&mut state) & 1 == 0;
            let end = 1000 + (next(&mut state) % 900) as i32;
            let start_loop = (next(&mut state) % 500) as i32;
            let end_loop =
                start_loop + 8 + (next(&mut state) % (end - start_loop - 8) as u64) as i32;
            // positions all along the sample, and a few just past the end of the loop or the sample
            let position = (next(&mut state) % (end_loop as u64 + 40)) as i64;
            let position_fp_extra = (next(&mut state) % Oscillator::FRAC_UNIT as u64) as i64;
            let ratio_fp = (next(&mut state) % (6 * Oscillator::FRAC_UNIT as u64)) as i64;
            let length = [1, 7, 64, 100, 1024][(next(&mut state) % 5) as usize];

            let make = || {
                let mut o = oscillator(looping, end, start_loop, end_loop, position);
                o.position_fp += position_fp_extra;
                o
            };
            let (mut fast, mut checked) = (make(), make());
            let (mut a, mut b) = (vec![0_f32; length], vec![0_f32; length]);
            let (ok_fast, ok_checked) = if looping {
                (
                    fast.fill_block_continuous(&data, &mut a, ratio_fp),
                    checked.fill_block_continuous_checked(&data, &mut b, ratio_fp),
                )
            } else {
                (
                    fast.fill_block_no_loop(&data, &mut a, ratio_fp),
                    checked.fill_block_no_loop_checked(&data, &mut b, ratio_fp),
                )
            };
            assert_eq!(ok_fast, ok_checked);
            if ok_fast {
                assert_eq!(a, b);
            }
            assert_eq!(fast.position_fp, checked.position_fp);
        }
    }

    // The cached pitch ratio must equal the one a fresh oscillator would compute, both when the pitch holds and when it changes.
    #[test]
    fn test_pitch_ratio_cache() {
        let data = vec![1000_i16; 4000];
        let make = || {
            let mut o = oscillator(true, 3000, 0, 2000, 0);
            o.sample_rate_ratio = 1.0;
            o.pitch_change_scale = 1.0;
            o.root_key = 60;
            o
        };
        let mut used = make();
        for pitch in [60.0_f32, 60.0, 60.0, 64.5, 64.5, 48.0, 60.0, 60.0] {
            let mut fresh = make();
            fresh.position_fp = used.position_fp;
            let (mut a, mut b) = (vec![0_f32; 64], vec![0_f32; 64]);
            assert!(used.process(&data, &mut a, pitch));
            assert!(fresh.process(&data, &mut b, pitch));
            assert_eq!(a, b);
            assert_eq!(used.position_fp, fresh.position_fp, "pitch {pitch}");
        }
    }
}
