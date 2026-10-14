#![allow(dead_code)]

use std::f32::consts;

use crate::synthesizer_settings::SynthesizerSettings;

/// A voice's filter paired with the sample block it filters, so that several voices can be filtered together.
pub(crate) type FilterAndBlock<'a> = (&'a mut BiQuadFilter, &'a mut [f32]);

#[derive(Debug)]
#[non_exhaustive]
pub(crate) struct BiQuadFilter {
    sample_rate: i32,

    active: bool,

    a0: f32,
    a1: f32,
    a2: f32,
    a3: f32,
    a4: f32,

    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl BiQuadFilter {
    const RESONANCE_PEAK_OFFSET: f32 = 1_f32 - 1_f32 / core::f32::consts::SQRT_2;

    pub(crate) fn new(settings: &SynthesizerSettings) -> Self {
        Self {
            sample_rate: settings.sample_rate,
            active: false,
            a0: 0_f32,
            a1: 0_f32,
            a2: 0_f32,
            a3: 0_f32,
            a4: 0_f32,
            x1: 0_f32,
            x2: 0_f32,
            y1: 0_f32,
            y2: 0_f32,
        }
    }

    pub(crate) fn clear_buffer(&mut self) {
        self.x1 = 0_f32;
        self.x2 = 0_f32;
        self.y1 = 0_f32;
        self.y2 = 0_f32;
    }

    pub(crate) fn set_low_pass_filter(&mut self, cutoff_frequency: f32, resonance: f32) {
        if cutoff_frequency < 0.499_f32 * self.sample_rate as f32 {
            self.active = true;

            // This equation gives the Q value which makes the desired resonance peak.
            // The error of the resultant peak height is less than 3%.
            let q = resonance
                - BiQuadFilter::RESONANCE_PEAK_OFFSET / (1_f32 + 6_f32 * (resonance - 1_f32));

            let w = 2_f32 * consts::PI * cutoff_frequency / self.sample_rate as f32;
            let cosw = w.cos();
            let alpha = w.sin() / (2_f32 * q);

            let b0 = (1_f32 - cosw) / 2_f32;
            let b1 = 1_f32 - cosw;
            let b2 = (1_f32 - cosw) / 2_f32;
            let a0 = 1_f32 + alpha;
            let a1 = -2_f32 * cosw;
            let a2 = 1_f32 - alpha;

            self.set_coefficients(a0, a1, a2, b0, b1, b2);
        } else {
            self.active = false;
        }
    }

    pub(crate) fn process(&mut self, block: &mut [f32]) {
        let block_length = block.len();

        if self.active {
            for input in block.iter_mut().take(block_length) {
                let output = self.a0 * *input + self.a1 * self.x1 + self.a2 * self.x2
                    - self.a3 * self.y1
                    - self.a4 * self.y2;

                self.x2 = self.x1;
                self.x1 = *input;
                self.y2 = self.y1;
                self.y1 = output;

                *input = output;
            }
        } else {
            self.x2 = block[block_length - 2];
            self.x1 = block[block_length - 1];
            self.y2 = self.x2;
            self.y1 = self.x1;
        }
    }

    /// Equivalent to calling `process` on each voice, but active filters are processed in groups of four via `process_four`. Does not allocate.
    pub(crate) fn process_many<'a>(voices: impl Iterator<Item = FilterAndBlock<'a>>) {
        let mut group: [Option<FilterAndBlock>; 4] = [None, None, None, None];
        let mut count = 0;
        for (filter, block) in voices {
            if !filter.active {
                filter.process(block);
                continue;
            }
            group[count] = Some((filter, block));
            count += 1;
            if count == 4 {
                if let [Some(a), Some(b), Some(c), Some(d)] = std::mem::take(&mut group) {
                    BiQuadFilter::process_four([a.0, b.0, c.0, d.0], [a.1, b.1, c.1, d.1]);
                }
                count = 0;
            }
        }
        for (filter, block) in group.into_iter().flatten() {
            filter.process(block);
        }
    }

    /// Processes four active filters over four blocks of the same length.
    /// The arithmetic and its order are the same as in `process`, so the results are bit-identical. Each filter's samples depend on the two before them,
    /// so one filter cannot be vectorized, but the four independent chains advance in lockstep and the compiler can use SIMD across them.
    ///
    /// Four lanes is what a single baseline x86-64 (SSE2) or ARM (NEON) register holds. Tried on 100 s of a black MIDI file:
    /// 8 and 16 lanes, and the same code compiled for AVX2 behind a run-time check, were no faster (the filter is no longer most of a voice's time),
    /// and hand-written 8 lanes were slower than 4 (more live values than registers).
    fn process_four(filters: [&mut BiQuadFilter; 4], blocks: [&mut [f32]; 4]) {
        let n = blocks[0].len();
        let [b0, b1, b2, b3] = blocks;
        let (mut a0, mut a1, mut a2, mut a3, mut a4) =
            ([0_f32; 4], [0_f32; 4], [0_f32; 4], [0_f32; 4], [0_f32; 4]);
        let (mut x1, mut x2, mut y1, mut y2) = ([0_f32; 4], [0_f32; 4], [0_f32; 4], [0_f32; 4]);
        for (l, f) in filters.iter().enumerate() {
            (a0[l], a1[l], a2[l], a3[l], a4[l]) = (f.a0, f.a1, f.a2, f.a3, f.a4);
            (x1[l], x2[l], y1[l], y2[l]) = (f.x1, f.x2, f.y1, f.y2);
        }
        for t in 0..n {
            let x = [b0[t], b1[t], b2[t], b3[t]];
            let y: [f32; 4] = std::array::from_fn(|l| {
                a0[l] * x[l] + a1[l] * x1[l] + a2[l] * x2[l] - a3[l] * y1[l] - a4[l] * y2[l]
            });
            x2 = x1;
            x1 = x;
            y2 = y1;
            y1 = y;
            (b0[t], b1[t], b2[t], b3[t]) = (y[0], y[1], y[2], y[3]);
        }
        for (l, f) in filters.into_iter().enumerate() {
            (f.x1, f.x2, f.y1, f.y2) = (x1[l], x2[l], y1[l], y2[l]);
        }
    }

    fn set_coefficients(&mut self, a0: f32, a1: f32, a2: f32, b0: f32, b1: f32, b2: f32) {
        self.a0 = b0 / a0;
        self.a1 = b1 / a0;
        self.a2 = b2 / a0;
        self.a3 = a1 / a0;
        self.a4 = a2 / a0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // a small generator, so that the test needs no dependency
    fn next(state: &mut u64) -> f32 {
        *state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((*state >> 40) as f32 / (1u64 << 24) as f32) * 2.0 - 1.0
    }

    // The batched filter must give the same samples and the same filter state, bit for bit, as filtering one by one.
    // The filters are a mix of active and inactive, with cutoffs and resonances that change from block to block
    // (as they do when a voice has a modulation envelope or LFO on its cutoff), and the voice counts straddle multiples of four.
    #[test]
    fn test_batched_filter_matches_single() {
        let settings = SynthesizerSettings::new(44100);
        let mut seed = 7_u64;
        for count in [0, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 33] {
            let mut one: Vec<(BiQuadFilter, Vec<f32>)> = (0..count)
                .map(|_| (BiQuadFilter::new(&settings), vec![0_f32; 64]))
                .collect();
            let mut many: Vec<(BiQuadFilter, Vec<f32>)> = (0..count)
                .map(|_| (BiQuadFilter::new(&settings), vec![0_f32; 64]))
                .collect();
            for _block in 0..12 {
                for i in 0..count {
                    // a cutoff from the bottom of the range to above the limit that switches the filter off, a resonance from flat to sharp
                    let cutoff = match next(&mut seed) {
                        x if x < -0.7 => 30000.0,
                        x => 40.0 + 12000.0 * (x + 1.0),
                    };
                    let resonance = 1.0 + 3.0 * (next(&mut seed) + 1.0);
                    one[i].0.set_low_pass_filter(cutoff, resonance);
                    many[i].0.set_low_pass_filter(cutoff, resonance);
                    // loud input, with silence now and then
                    let silent = next(&mut seed) > 0.8;
                    for t in 0..64 {
                        let x = if silent { 0.0 } else { 4.0 * next(&mut seed) };
                        one[i].1[t] = x;
                        many[i].1[t] = x;
                    }
                }
                for (f, b) in one.iter_mut() {
                    f.process(b);
                }
                BiQuadFilter::process_many(many.iter_mut().map(|(f, b)| (f, &mut b[..])));
                for (i, ((fa, a), (fb, b))) in one.iter().zip(many.iter()).enumerate() {
                    let bits = |v: &[f32]| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
                    assert_eq!(bits(a), bits(b), "{count} voices, voice {i}");
                    assert_eq!(
                        [fa.x1, fa.x2, fa.y1, fa.y2].map(f32::to_bits),
                        [fb.x1, fb.x2, fb.y1, fb.y2].map(f32::to_bits),
                        "{count} voices, state of voice {i}"
                    );
                }
            }
        }
    }
}
