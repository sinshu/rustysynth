#![allow(dead_code)]

use std::f32::consts;

use crate::synthesizer_settings::SynthesizerSettings;

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

    fn set_coefficients(&mut self, a0: f32, a1: f32, a2: f32, b0: f32, b1: f32, b2: f32) {
        self.a0 = b0 / a0;
        self.a1 = b1 / a0;
        self.a2 = b2 / a0;
        self.a3 = a1 / a0;
        self.a4 = a2 / a0;
    }
}
