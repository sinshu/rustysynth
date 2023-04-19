#![allow(dead_code)]

use crate::synthesizer_settings::SynthesizerSettings;

#[non_exhaustive]
pub(crate) struct Lfo {
    sample_rate: i32,
    block_size: usize,

    active: bool,

    delay: f64,
    period: f64,

    processed_sample_count: usize,
    value: f32,
}

impl Lfo {
    pub(crate) fn new(settings: &SynthesizerSettings) -> Self {
        Self {
            sample_rate: settings.sample_rate,
            block_size: settings.block_size,
            active: false,
            delay: 0_f64,
            period: 0_f64,
            processed_sample_count: 0,
            value: 0_f32,
        }
    }

    pub(crate) fn start(&mut self, delay: f32, frequency: f32) {
        if frequency > 1.0E-3_f32 {
            self.active = true;

            self.delay = delay as f64;
            self.period = 1.0_f64 / frequency as f64;

            self.processed_sample_count = 0;
            self.value = 0_f32;
        } else {
            self.active = false;
            self.value = 0_f32;
        }
    }

    pub(crate) fn process(&mut self) {
        if !self.active {
            return;
        }

        self.processed_sample_count += self.block_size;

        let current_time = self.processed_sample_count as f64 / self.sample_rate as f64;

        if current_time < self.delay {
            self.value = 0_f32;
        } else {
            let phase = ((current_time - self.delay) % self.period) / self.period;
            if phase < 0.25 {
                self.value = (4_f64 * phase) as f32;
            } else if phase < 0.75 {
                self.value = (4_f64 * (0.5 - phase)) as f32;
            } else {
                self.value = (4_f64 * (phase - 1.0)) as f32;
            }
        }
    }

    pub(crate) fn get_value(&self) -> f32 {
        self.value
    }
}
