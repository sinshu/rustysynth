#![allow(dead_code)]

use crate::error::SoundFontError;
use crate::generator::Generator;
use crate::generator_type::GeneratorType;
use crate::loop_mode::LoopMode;
use crate::sample_header::SampleHeader;
use crate::soundfont_math::SoundFontMath;
use crate::zone::Zone;

fn set_parameter(gs: &mut [i16; GeneratorType::COUNT], generator: &Generator) {
    let index = generator.generator_type as usize;

    // Unknown generators should be ignored.
    if index < gs.len() {
        gs[index] = generator.value as i16;
    }
}

#[non_exhaustive]
pub struct InstrumentRegion {
    pub(crate) gs: [i16; GeneratorType::COUNT],
    pub(crate) sample_start: i32,
    pub(crate) sample_end: i32,
    pub(crate) sample_start_loop: i32,
    pub(crate) sample_end_loop: i32,
    pub(crate) sample_sample_rate: i32,
    pub(crate) sample_original_pitch: i32,
    pub(crate) sample_pitch_correction: i32,
}

impl InstrumentRegion {
    fn new(
        name: &str,
        global: &Zone,
        local: &Zone,
        samples: &[SampleHeader],
    ) -> Result<Self, SoundFontError> {
        let mut gs: [i16; GeneratorType::COUNT] = [0; GeneratorType::COUNT];
        gs[GeneratorType::INITIAL_FILTER_CUTOFF_FREQUENCY as usize] = 13500;
        gs[GeneratorType::DELAY_MODULATION_LFO as usize] = -12000;
        gs[GeneratorType::DELAY_VIBRATO_LFO as usize] = -12000;
        gs[GeneratorType::DELAY_MODULATION_ENVELOPE as usize] = -12000;
        gs[GeneratorType::ATTACK_MODULATION_ENVELOPE as usize] = -12000;
        gs[GeneratorType::HOLD_MODULATION_ENVELOPE as usize] = -12000;
        gs[GeneratorType::DECAY_MODULATION_ENVELOPE as usize] = -12000;
        gs[GeneratorType::RELEASE_MODULATION_ENVELOPE as usize] = -12000;
        gs[GeneratorType::DELAY_VOLUME_ENVELOPE as usize] = -12000;
        gs[GeneratorType::ATTACK_VOLUME_ENVELOPE as usize] = -12000;
        gs[GeneratorType::HOLD_VOLUME_ENVELOPE as usize] = -12000;
        gs[GeneratorType::DECAY_VOLUME_ENVELOPE as usize] = -12000;
        gs[GeneratorType::RELEASE_VOLUME_ENVELOPE as usize] = -12000;
        gs[GeneratorType::KEY_RANGE as usize] = 0x7F00;
        gs[GeneratorType::VELOCITY_RANGE as usize] = 0x7F00;
        gs[GeneratorType::KEY_NUMBER as usize] = -1;
        gs[GeneratorType::VELOCITY as usize] = -1;
        gs[GeneratorType::SCALE_TUNING as usize] = 100;
        gs[GeneratorType::OVERRIDING_ROOT_KEY as usize] = -1;

        for generator in global.generators.iter() {
            set_parameter(&mut gs, generator);
        }

        for generator in local.generators.iter() {
            set_parameter(&mut gs, generator);
        }

        let id = gs[GeneratorType::SAMPLE_ID as usize] as usize;
        if id >= samples.len() {
            return Err(SoundFontError::InvalidSampleId {
                instrument_name: name.to_string(),
                sample_id: id,
            });
        }
        let sample = &samples[id];

        Ok(Self {
            gs,
            sample_start: sample.start,
            sample_end: sample.end,
            sample_start_loop: sample.start_loop,
            sample_end_loop: sample.end_loop,
            sample_sample_rate: sample.sample_rate,
            sample_original_pitch: sample.original_pitch as i32,
            sample_pitch_correction: sample.pitch_correction as i32,
        })
    }

    pub(crate) fn create(
        name: &str,
        zones: &[Zone],
        samples: &[SampleHeader],
    ) -> Result<Vec<InstrumentRegion>, SoundFontError> {
        // Is the first one the global zone?
        if zones[0].generators.is_empty()
            || zones[0].generators.last().unwrap().generator_type != GeneratorType::SAMPLE_ID
        {
            // The first one is the global zone.
            let global = &zones[0];

            // The global zone is regarded as the base setting of subsequent zones.
            let count = zones.len() - 1;
            let mut regions: Vec<InstrumentRegion> = Vec::new();
            for i in 0..count {
                regions.push(InstrumentRegion::new(name, global, &zones[i + 1], samples)?);
            }

            Ok(regions)
        } else {
            // No global zone.
            let count = zones.len();
            let mut regions: Vec<InstrumentRegion> = Vec::new();
            for zone in zones.iter().take(count) {
                regions.push(InstrumentRegion::new(name, &Zone::empty(), zone, samples)?);
            }

            Ok(regions)
        }
    }

    pub fn contains(&self, key: i32, velocity: i32) -> bool {
        let contains_key = self.get_key_range_start() <= key && key <= self.get_key_range_end();
        let contains_velocity = self.get_velocity_range_start() <= velocity
            && velocity <= self.get_velocity_range_end();
        contains_key && contains_velocity
    }

    pub fn get_sample_start(&self) -> i32 {
        self.sample_start + self.get_start_address_offset()
    }

    pub fn get_sample_end(&self) -> i32 {
        self.sample_end + self.get_end_address_offset()
    }

    pub fn get_sample_start_loop(&self) -> i32 {
        self.sample_start_loop + self.get_start_loop_address_offset()
    }

    pub fn get_sample_end_loop(&self) -> i32 {
        self.sample_end_loop + self.get_end_loop_address_offset()
    }

    pub fn get_start_address_offset(&self) -> i32 {
        32768 * self.gs[GeneratorType::START_ADDRESS_COARSE_OFFSET as usize] as i32
            + self.gs[GeneratorType::START_ADDRESS_OFFSET as usize] as i32
    }

    pub fn get_end_address_offset(&self) -> i32 {
        32768 * self.gs[GeneratorType::END_ADDRESS_COARSE_OFFSET as usize] as i32
            + self.gs[GeneratorType::END_ADDRESS_OFFSET as usize] as i32
    }

    pub fn get_start_loop_address_offset(&self) -> i32 {
        32768 * self.gs[GeneratorType::START_LOOP_ADDRESS_COARSE_OFFSET as usize] as i32
            + self.gs[GeneratorType::START_LOOP_ADDRESS_OFFSET as usize] as i32
    }

    pub fn get_end_loop_address_offset(&self) -> i32 {
        32768 * self.gs[GeneratorType::END_LOOP_ADDRESS_COARSE_OFFSET as usize] as i32
            + self.gs[GeneratorType::END_LOOP_ADDRESS_OFFSET as usize] as i32
    }

    pub fn get_modulation_lfo_to_pitch(&self) -> i32 {
        self.gs[GeneratorType::MODULATION_LFO_TO_PITCH as usize] as i32
    }

    pub fn get_vibrato_lfo_to_pitch(&self) -> i32 {
        self.gs[GeneratorType::VIBRATO_LFO_TO_PITCH as usize] as i32
    }

    pub fn get_modulation_envelope_to_pitch(&self) -> i32 {
        self.gs[GeneratorType::MODULATION_ENVELOPE_TO_PITCH as usize] as i32
    }

    pub fn get_initial_filter_cutoff_frequency(&self) -> f32 {
        SoundFontMath::cents_to_hertz(
            self.gs[GeneratorType::INITIAL_FILTER_CUTOFF_FREQUENCY as usize] as f32,
        )
    }

    pub fn get_initial_filter_q(&self) -> f32 {
        0.1_f32 * self.gs[GeneratorType::INITIAL_FILTER_Q as usize] as f32
    }

    pub fn get_modulation_lfo_to_filter_cutoff_frequency(&self) -> i32 {
        self.gs[GeneratorType::MODULATION_LFO_TO_FILTER_CUTOFF_FREQUENCY as usize] as i32
    }

    pub fn get_modulation_envelope_to_filter_cutoff_frequency(&self) -> i32 {
        self.gs[GeneratorType::MODULATION_ENVELOPE_TO_FILTER_CUTOFF_FREQUENCY as usize] as i32
    }

    pub fn get_modulation_lfo_to_volume(&self) -> f32 {
        0.1_f32 * self.gs[GeneratorType::MODULATION_LFO_TO_VOLUME as usize] as f32
    }

    pub fn get_chorus_effects_send(&self) -> f32 {
        0.1_f32 * self.gs[GeneratorType::CHORUS_EFFECTS_SEND as usize] as f32
    }

    pub fn get_reverb_effects_send(&self) -> f32 {
        0.1_f32 * self.gs[GeneratorType::REVERB_EFFECTS_SEND as usize] as f32
    }

    pub fn get_pan(&self) -> f32 {
        0.1_f32 * self.gs[GeneratorType::PAN as usize] as f32
    }

    pub fn get_delay_modulation_lfo(&self) -> f32 {
        SoundFontMath::timecents_to_seconds(
            self.gs[GeneratorType::DELAY_MODULATION_LFO as usize] as f32,
        )
    }

    pub fn get_frequency_modulation_lfo(&self) -> f32 {
        SoundFontMath::cents_to_hertz(
            self.gs[GeneratorType::FREQUENCY_MODULATION_LFO as usize] as f32,
        )
    }

    pub fn get_delay_vibrato_lfo(&self) -> f32 {
        SoundFontMath::timecents_to_seconds(
            self.gs[GeneratorType::DELAY_VIBRATO_LFO as usize] as f32,
        )
    }

    pub fn get_frequency_vibrato_lfo(&self) -> f32 {
        SoundFontMath::cents_to_hertz(self.gs[GeneratorType::FREQUENCY_VIBRATO_LFO as usize] as f32)
    }

    pub fn get_delay_modulation_envelope(&self) -> f32 {
        SoundFontMath::timecents_to_seconds(
            self.gs[GeneratorType::DELAY_MODULATION_ENVELOPE as usize] as f32,
        )
    }

    pub fn get_attack_modulation_envelope(&self) -> f32 {
        SoundFontMath::timecents_to_seconds(
            self.gs[GeneratorType::ATTACK_MODULATION_ENVELOPE as usize] as f32,
        )
    }

    pub fn get_hold_modulation_envelope(&self) -> f32 {
        SoundFontMath::timecents_to_seconds(
            self.gs[GeneratorType::HOLD_MODULATION_ENVELOPE as usize] as f32,
        )
    }

    pub fn get_decay_modulation_envelope(&self) -> f32 {
        SoundFontMath::timecents_to_seconds(
            self.gs[GeneratorType::DECAY_MODULATION_ENVELOPE as usize] as f32,
        )
    }

    pub fn get_sustain_modulation_envelope(&self) -> f32 {
        0.1_f32 * self.gs[GeneratorType::SUSTAIN_MODULATION_ENVELOPE as usize] as f32
    }

    pub fn get_release_modulation_envelope(&self) -> f32 {
        SoundFontMath::timecents_to_seconds(
            self.gs[GeneratorType::RELEASE_MODULATION_ENVELOPE as usize] as f32,
        )
    }

    pub fn get_key_number_to_modulation_envelope_hold(&self) -> i32 {
        self.gs[GeneratorType::KEY_NUMBER_TO_MODULATION_ENVELOPE_HOLD as usize] as i32
    }

    pub fn get_key_number_to_modulation_envelope_decay(&self) -> i32 {
        self.gs[GeneratorType::KEY_NUMBER_TO_MODULATION_ENVELOPE_DECAY as usize] as i32
    }

    pub fn get_delay_volume_envelope(&self) -> f32 {
        SoundFontMath::timecents_to_seconds(
            self.gs[GeneratorType::DELAY_VOLUME_ENVELOPE as usize] as f32,
        )
    }

    pub fn get_attack_volume_envelope(&self) -> f32 {
        SoundFontMath::timecents_to_seconds(
            self.gs[GeneratorType::ATTACK_VOLUME_ENVELOPE as usize] as f32,
        )
    }

    pub fn get_hold_volume_envelope(&self) -> f32 {
        SoundFontMath::timecents_to_seconds(
            self.gs[GeneratorType::HOLD_VOLUME_ENVELOPE as usize] as f32,
        )
    }

    pub fn get_decay_volume_envelope(&self) -> f32 {
        SoundFontMath::timecents_to_seconds(
            self.gs[GeneratorType::DECAY_VOLUME_ENVELOPE as usize] as f32,
        )
    }

    pub fn get_sustain_volume_envelope(&self) -> f32 {
        0.1_f32 * self.gs[GeneratorType::SUSTAIN_VOLUME_ENVELOPE as usize] as f32
    }

    pub fn get_release_volume_envelope(&self) -> f32 {
        SoundFontMath::timecents_to_seconds(
            self.gs[GeneratorType::RELEASE_VOLUME_ENVELOPE as usize] as f32,
        )
    }

    pub fn get_key_number_to_volume_envelope_hold(&self) -> i32 {
        self.gs[GeneratorType::KEY_NUMBER_TO_VOLUME_ENVELOPE_HOLD as usize] as i32
    }

    pub fn get_key_number_to_volume_envelope_decay(&self) -> i32 {
        self.gs[GeneratorType::KEY_NUMBER_TO_VOLUME_ENVELOPE_DECAY as usize] as i32
    }

    pub fn get_key_range_start(&self) -> i32 {
        self.gs[GeneratorType::KEY_RANGE as usize] as i32 & 0xFF
    }

    pub fn get_key_range_end(&self) -> i32 {
        (self.gs[GeneratorType::KEY_RANGE as usize] as i32 >> 8) & 0xFF
    }

    pub fn get_velocity_range_start(&self) -> i32 {
        self.gs[GeneratorType::VELOCITY_RANGE as usize] as i32 & 0xFF
    }

    pub fn get_velocity_range_end(&self) -> i32 {
        (self.gs[GeneratorType::VELOCITY_RANGE as usize] as i32 >> 8) & 0xFF
    }

    pub fn get_initial_attenuation(&self) -> f32 {
        0.1_f32 * self.gs[GeneratorType::INITIAL_ATTENUATION as usize] as f32
    }

    pub fn get_coarse_tune(&self) -> i32 {
        self.gs[GeneratorType::COARSE_TUNE as usize] as i32
    }

    pub fn get_fine_tune(&self) -> i32 {
        self.gs[GeneratorType::FINE_TUNE as usize] as i32 + self.sample_pitch_correction
    }

    pub fn get_sample_modes(&self) -> i32 {
        if self.gs[GeneratorType::SAMPLE_MODES as usize] != 2 {
            self.gs[GeneratorType::SAMPLE_MODES as usize] as i32
        } else {
            LoopMode::NO_LOOP
        }
    }

    pub fn get_scale_tuning(&self) -> i32 {
        self.gs[GeneratorType::SCALE_TUNING as usize] as i32
    }

    pub fn get_exclusive_class(&self) -> i32 {
        self.gs[GeneratorType::EXCLUSIVE_CLASS as usize] as i32
    }

    pub fn get_root_key(&self) -> i32 {
        if self.gs[GeneratorType::OVERRIDING_ROOT_KEY as usize] != -1 {
            self.gs[GeneratorType::OVERRIDING_ROOT_KEY as usize] as i32
        } else {
            self.sample_original_pitch
        }
    }
}
