#![allow(dead_code)]

use crate::envelope_stage::EnvelopeStage;
use crate::soundfont_math::SoundFontMath;
use crate::synthesizer_settings::SynthesizerSettings;

#[non_exhaustive]
pub(crate) struct VolumeEnvelope {
    sample_rate: i32,

    attack_slope: f64,
    decay_slope: f64,
    release_slope: f64,

    attack_start_time: f64,
    hold_start_time: f64,
    decay_start_time: f64,
    release_start_time: f64,

    sustain_level: f32,
    release_level: f32,

    processed_sample_count: usize,
    stage: i32,
    value: f32,

    priority: f32,
}

impl VolumeEnvelope {
    pub(crate) fn new(settings: &SynthesizerSettings) -> Self {
        Self {
            sample_rate: settings.sample_rate,
            attack_slope: 0_f64,
            decay_slope: 0_f64,
            release_slope: 0_f64,
            attack_start_time: 0_f64,
            hold_start_time: 0_f64,
            decay_start_time: 0_f64,
            release_start_time: 0_f64,
            sustain_level: 0_f32,
            release_level: 0_f32,
            processed_sample_count: 0,
            stage: 0,
            value: 0_f32,
            priority: 0_f32,
        }
    }

    pub(crate) fn start(
        &mut self,
        delay: f32,
        attack: f32,
        hold: f32,
        decay: f32,
        sustain: f32,
        release: f32,
    ) {
        self.attack_slope = 1_f64 / attack as f64;
        self.decay_slope = -9.226_f64 / decay as f64;
        self.release_slope = -9.226_f64 / release as f64;

        self.attack_start_time = delay as f64;
        self.hold_start_time = self.attack_start_time + attack as f64;
        self.decay_start_time = self.hold_start_time + hold as f64;
        self.release_start_time = 0_f64;

        self.sustain_level = SoundFontMath::clamp(sustain, 0_f32, 1_f32);
        self.release_level = 0_f32;

        self.processed_sample_count = 0;
        self.stage = EnvelopeStage::DELAY;
        self.value = 0_f32;

        self.process(0);
    }

    pub(crate) fn release(&mut self) {
        self.stage = EnvelopeStage::RELEASE;
        self.release_start_time = self.processed_sample_count as f64 / self.sample_rate as f64;
        self.release_level = self.value;
    }

    pub(crate) fn process(&mut self, sample_count: usize) -> bool {
        self.processed_sample_count += sample_count;

        let current_time = self.processed_sample_count as f64 / self.sample_rate as f64;

        while self.stage <= EnvelopeStage::HOLD {
            let end_time = match self.stage {
                EnvelopeStage::DELAY => self.attack_start_time,
                EnvelopeStage::ATTACK => self.hold_start_time,
                EnvelopeStage::HOLD => self.decay_start_time,
                _ => panic!("Invalid envelope stage."),
            };

            if current_time < end_time {
                break;
            } else {
                self.stage += 1;
            }
        }

        if self.stage == EnvelopeStage::DELAY {
            self.value = 0_f32;
            self.priority = 4_f32 + self.value;
            true
        } else if self.stage == EnvelopeStage::ATTACK {
            self.value = (self.attack_slope * (current_time - self.attack_start_time)) as f32;
            self.priority = 3_f32 + self.value;
            true
        } else if self.stage == EnvelopeStage::HOLD {
            self.value = 1_f32;
            self.priority = 2_f32 + self.value;
            true
        } else if self.stage == EnvelopeStage::DECAY {
            self.value = SoundFontMath::max(
                SoundFontMath::exp_cutoff(self.decay_slope * (current_time - self.decay_start_time))
                    as f32,
                self.sustain_level,
            );
            self.priority = 1_f32 + self.value;
            self.value > SoundFontMath::NON_AUDIBLE
        } else if self.stage == EnvelopeStage::RELEASE {
            self.value = (self.release_level as f64
                * SoundFontMath::exp_cutoff(
                    self.release_slope * (current_time - self.release_start_time),
                )) as f32;
            self.priority = self.value;
            self.value > SoundFontMath::NON_AUDIBLE
        } else {
            panic!("Invalid envelope stage.");
        }
    }

    pub(crate) fn get_value(&self) -> f32 {
        self.value
    }

    pub(crate) fn get_priority(&self) -> f32 {
        self.priority
    }
}
