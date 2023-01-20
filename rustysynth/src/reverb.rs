#![allow(dead_code)]

use std::cmp;

#[non_exhaustive]
pub(crate) struct Reverb {
    cfs_l: Vec<CombFilter>,
    cfs_r: Vec<CombFilter>,
    apfs_l: Vec<AllPassFilter>,
    apfs_r: Vec<AllPassFilter>,

    gain: f32,
    room_size: f32,
    room_size1: f32,
    damp: f32,
    damp1: f32,
    wet: f32,
    wet1: f32,
    wet2: f32,
    width: f32,
}

impl Reverb {
    const FIXED_GAIN: f32 = 0.015;
    const SCALE_WET: f32 = 3.0;
    const SCALE_DAMP: f32 = 0.4;
    const SCALE_ROOM: f32 = 0.28;
    const OFFSET_ROOM: f32 = 0.7;
    const INITIAL_ROOM: f32 = 0.5;
    const INITIAL_DAMP: f32 = 0.5;
    const INITIAL_WET: f32 = 1.0 / Reverb::SCALE_WET;
    const INITIAL_WIDTH: f32 = 1.0;
    const STEREO_SPREAD: usize = 23;

    const CF_TUNING_L1: usize = 1116;
    const CF_TUNING_R1: usize = 1116 + Reverb::STEREO_SPREAD;
    const CF_TUNING_L2: usize = 1188;
    const CF_TUNING_R2: usize = 1188 + Reverb::STEREO_SPREAD;
    const CF_TUNING_L3: usize = 1277;
    const CF_TUNING_R3: usize = 1277 + Reverb::STEREO_SPREAD;
    const CF_TUNING_L4: usize = 1356;
    const CF_TUNING_R4: usize = 1356 + Reverb::STEREO_SPREAD;
    const CF_TUNING_L5: usize = 1422;
    const CF_TUNING_R5: usize = 1422 + Reverb::STEREO_SPREAD;
    const CF_TUNING_L6: usize = 1491;
    const CF_TUNING_R6: usize = 1491 + Reverb::STEREO_SPREAD;
    const CF_TUNING_L7: usize = 1557;
    const CF_TUNING_R7: usize = 1557 + Reverb::STEREO_SPREAD;
    const CF_TUNING_L8: usize = 1617;
    const CF_TUNING_R8: usize = 1617 + Reverb::STEREO_SPREAD;
    const APF_TUNING_L1: usize = 556;
    const APF_TUNING_R1: usize = 556 + Reverb::STEREO_SPREAD;
    const APF_TUNING_L2: usize = 441;
    const APF_TUNING_R2: usize = 441 + Reverb::STEREO_SPREAD;
    const APF_TUNING_L3: usize = 341;
    const APF_TUNING_R3: usize = 341 + Reverb::STEREO_SPREAD;
    const APF_TUNING_L4: usize = 225;
    const APF_TUNING_R4: usize = 225 + Reverb::STEREO_SPREAD;

    pub(crate) fn new(sample_rate: i32) -> Self {
        let mut cfs_l: Vec<CombFilter> = Vec::new();
        cfs_l.push(CombFilter::new(Reverb::CF_TUNING_L1));
        cfs_l.push(CombFilter::new(Reverb::CF_TUNING_L2));
        cfs_l.push(CombFilter::new(Reverb::CF_TUNING_L3));
        cfs_l.push(CombFilter::new(Reverb::CF_TUNING_L4));
        cfs_l.push(CombFilter::new(Reverb::CF_TUNING_L5));
        cfs_l.push(CombFilter::new(Reverb::CF_TUNING_L6));
        cfs_l.push(CombFilter::new(Reverb::CF_TUNING_L7));
        cfs_l.push(CombFilter::new(Reverb::CF_TUNING_L8));

        let mut cfs_r: Vec<CombFilter> = Vec::new();
        cfs_r.push(CombFilter::new(Reverb::CF_TUNING_R1));
        cfs_r.push(CombFilter::new(Reverb::CF_TUNING_R2));
        cfs_r.push(CombFilter::new(Reverb::CF_TUNING_R3));
        cfs_r.push(CombFilter::new(Reverb::CF_TUNING_R4));
        cfs_r.push(CombFilter::new(Reverb::CF_TUNING_R5));
        cfs_r.push(CombFilter::new(Reverb::CF_TUNING_R6));
        cfs_r.push(CombFilter::new(Reverb::CF_TUNING_R7));
        cfs_r.push(CombFilter::new(Reverb::CF_TUNING_R8));

        let mut apfs_l : Vec<AllPassFilter> = Vec::new();
        apfs_l.push(AllPassFilter::new(Reverb::APF_TUNING_L1));
        apfs_l.push(AllPassFilter::new(Reverb::APF_TUNING_L2));
        apfs_l.push(AllPassFilter::new(Reverb::APF_TUNING_L3));
        apfs_l.push(AllPassFilter::new(Reverb::APF_TUNING_L4));

        let mut apfs_r : Vec<AllPassFilter> = Vec::new();
        apfs_r.push(AllPassFilter::new(Reverb::APF_TUNING_R1));
        apfs_r.push(AllPassFilter::new(Reverb::APF_TUNING_R2));
        apfs_r.push(AllPassFilter::new(Reverb::APF_TUNING_R3));
        apfs_r.push(AllPassFilter::new(Reverb::APF_TUNING_R4));

        let gain: f32 = 0.0;
        let room_size: f32 = Reverb::INITIAL_ROOM;
        let room_size1: f32 = 0.0;
        let damp: f32 = Reverb::INITIAL_DAMP;
        let damp1: f32 = 0.0;
        let wet: f32 = Reverb::INITIAL_WET;
        let wet1: f32 = 0.0;
        let wet2: f32 = 0.0;
        let width: f32 = Reverb::INITIAL_WIDTH;

        Self {
            cfs_l: cfs_l,
            cfs_r: cfs_r,
            apfs_l: apfs_l,
            apfs_r: apfs_r,
            gain: gain,
            room_size: room_size,
            room_size1: room_size1,
            damp: damp,
            damp1: damp1,
            wet: wet,
            wet1: wet1,
            wet2: wet2,
            width: width
        }
    }
}

#[non_exhaustive]
struct CombFilter {
    buffer: Vec<f32>,

    buffer_index: usize,
    filter_store: f32,

    feedback: f32,
    damp1: f32,
    damp2: f32,
}

impl CombFilter {
    fn new(buffer_size: usize) -> Self {
        Self {
            buffer: vec![0_f32; buffer_size],
            buffer_index: 0,
            filter_store: 0_f32,
            feedback: 0_f32,
            damp1: 0_f32,
            damp2: 0_f32,
        }
    }

    fn mute(&mut self) {
        let buffer_length = self.buffer.len();
        for i in 0..buffer_length {
            self.buffer[i] = 0_f32;
        }

        self.filter_store = 0_f32;
    }

    fn process(&mut self, input_block: &[f32], output_block: &mut [f32]) {
        let buffer_length = self.buffer.len();
        let output_block_length = output_block.len();
        
        let mut block_index: usize = 0;
        while block_index < output_block_length {
            if self.buffer_index == buffer_length {
                self.buffer_index = 0;
            }

            let src_rem = buffer_length - self.buffer_index;
            let dst_rem = output_block_length - block_index;
            let rem = cmp::min(src_rem, dst_rem);

            for t in 0..rem {
                let block_pos = block_index + t;
                let buffer_pos = self.buffer_index + t;

                let input = input_block[block_pos];

                // The following ifs are to avoid performance problem due to denormalized number.
                // The original implementation uses unsafe cast to detect denormalized number.
                // I tried to reproduce the original implementation using Unsafe.As,
                // but the simple Math.Abs version was faster according to some benchmarks.

                let mut output = self.buffer[buffer_pos];
                if output.abs() < 1.0E-6_f32 {
                    output = 0_f32;
                }

                self.filter_store = (output * self.damp2) + (self.filter_store * self.damp1);
                if self.filter_store.abs() < 1.0E-6_f32 {
                    self.filter_store = 0_f32;
                }

                self.buffer[buffer_pos] = input + (self.filter_store * self.feedback);
                output_block[block_pos] += output;
            }

            self.buffer_index += rem;
            block_index += rem;
        }
    }
}

#[non_exhaustive]
struct AllPassFilter {
    buffer: Vec<f32>,

    buffer_index: usize,

    feedback: f32,
}

impl AllPassFilter {
    fn new(buffer_size: usize) -> Self {
        Self {
            buffer: vec![0_f32; buffer_size],
            buffer_index: 0,
            feedback: 0_f32,
        }
    }

    fn mute(&mut self) {
        let buffer_length = self.buffer.len();
        for i in 0..buffer_length {
            self.buffer[i] = 0_f32;
        }
    }

    fn process(&mut self, block: &mut [f32]) {
        let buffer_length = self.buffer.len();
        let block_length = block.len();

        let mut block_index: usize = 0;
        while block_index < block_length {
            if self.buffer_index == buffer_length {
                self.buffer_index = 0;
            }

            let src_rem = buffer_length - self.buffer_index;
            let dst_rem = block_length - block_index;
            let rem = cmp::min(src_rem, dst_rem);

            for t in 0..rem {
                let block_pos = block_index + t;
                let buffer_pos = self.buffer_index + t;

                let input = block[block_pos];

                let mut bufout = self.buffer[buffer_pos];
                if bufout.abs() < 1.0E-6_f32 {
                    bufout = 0_f32;
                }

                block[block_pos] = bufout - input;
                self.buffer[buffer_pos] = input + (bufout * self.feedback);
            }

            self.buffer_index += rem;
            block_index += rem;
        }
    }
}
