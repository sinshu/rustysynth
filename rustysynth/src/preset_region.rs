#![allow(dead_code)]

use crate::error::SoundFontError;
use crate::generator::Generator;
use crate::generator_type::GeneratorType;
use crate::instrument::Instrument;
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
pub struct PresetRegion {
    pub(crate) gs: [i16; GeneratorType::COUNT],
    pub(crate) instrument: usize,
}

impl PresetRegion {
    fn new(
        name: &str,
        global: &Zone,
        local: &Zone,
        samples: &[Instrument],
    ) -> Result<Self, SoundFontError> {
        let mut gs: [i16; GeneratorType::COUNT] = [0; GeneratorType::COUNT];
        gs[GeneratorType::KEY_RANGE as usize] = 0x7F00;
        gs[GeneratorType::VELOCITY_RANGE as usize] = 0x7F00;

        for generator in global.generators.iter() {
            set_parameter(&mut gs, generator);
        }

        for generator in local.generators.iter() {
            set_parameter(&mut gs, generator);
        }

        let id = gs[GeneratorType::INSTRUMENT as usize] as usize;
        if id >= samples.len() {
            return Err(SoundFontError::InvalidInstrumentId {
                preset_name: name.to_string(),
                instrument_id: id,
            });
        }

        Ok(Self { gs, instrument: id })
    }

    pub(crate) fn create(
        name: &str,
        zones: &[Zone],
        instruments: &[Instrument],
    ) -> Result<Vec<PresetRegion>, SoundFontError> {
        // Is the first one the global zone?
        if zones[0].generators.is_empty()
            || zones[0].generators.last().unwrap().generator_type != GeneratorType::INSTRUMENT
        {
            // The first one is the global zone.
            let global = &zones[0];

            // The global zone is regarded as the base setting of subsequent zones.
            let count = zones.len() - 1;
            let mut regions: Vec<PresetRegion> = Vec::new();
            for i in 0..count {
                regions.push(PresetRegion::new(name, global, &zones[i + 1], instruments)?);
            }

            Ok(regions)
        } else {
            // No global zone.
            let count = zones.len();
            let mut regions: Vec<PresetRegion> = Vec::new();
            for zone in zones.iter().take(count) {
                regions.push(PresetRegion::new(name, &Zone::empty(), zone, instruments)?);
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
        SoundFontMath::cents_to_multiplying_factor(
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
        SoundFontMath::cents_to_multiplying_factor(
            self.gs[GeneratorType::DELAY_MODULATION_LFO as usize] as f32,
        )
    }

    pub fn get_frequency_modulation_lfo(&self) -> f32 {
        SoundFontMath::cents_to_multiplying_factor(
            self.gs[GeneratorType::FREQUENCY_MODULATION_LFO as usize] as f32,
        )
    }

    pub fn get_delay_vibrato_lfo(&self) -> f32 {
        SoundFontMath::cents_to_multiplying_factor(
            self.gs[GeneratorType::DELAY_VIBRATO_LFO as usize] as f32,
        )
    }

    pub fn get_frequency_vibrato_lfo(&self) -> f32 {
        SoundFontMath::cents_to_multiplying_factor(
            self.gs[GeneratorType::FREQUENCY_VIBRATO_LFO as usize] as f32,
        )
    }

    pub fn get_delay_modulation_envelope(&self) -> f32 {
        SoundFontMath::cents_to_multiplying_factor(
            self.gs[GeneratorType::DELAY_MODULATION_ENVELOPE as usize] as f32,
        )
    }

    pub fn get_attack_modulation_envelope(&self) -> f32 {
        SoundFontMath::cents_to_multiplying_factor(
            self.gs[GeneratorType::ATTACK_MODULATION_ENVELOPE as usize] as f32,
        )
    }

    pub fn get_hold_modulation_envelope(&self) -> f32 {
        SoundFontMath::cents_to_multiplying_factor(
            self.gs[GeneratorType::HOLD_MODULATION_ENVELOPE as usize] as f32,
        )
    }

    pub fn get_decay_modulation_envelope(&self) -> f32 {
        SoundFontMath::cents_to_multiplying_factor(
            self.gs[GeneratorType::DECAY_MODULATION_ENVELOPE as usize] as f32,
        )
    }

    pub fn get_sustain_modulation_envelope(&self) -> f32 {
        0.1_f32 * self.gs[GeneratorType::SUSTAIN_MODULATION_ENVELOPE as usize] as f32
    }

    pub fn get_release_modulation_envelope(&self) -> f32 {
        SoundFontMath::cents_to_multiplying_factor(
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
        SoundFontMath::cents_to_multiplying_factor(
            self.gs[GeneratorType::DELAY_VOLUME_ENVELOPE as usize] as f32,
        )
    }

    pub fn get_attack_volume_envelope(&self) -> f32 {
        SoundFontMath::cents_to_multiplying_factor(
            self.gs[GeneratorType::ATTACK_VOLUME_ENVELOPE as usize] as f32,
        )
    }

    pub fn get_hold_volume_envelope(&self) -> f32 {
        SoundFontMath::cents_to_multiplying_factor(
            self.gs[GeneratorType::HOLD_VOLUME_ENVELOPE as usize] as f32,
        )
    }

    pub fn get_decay_volume_envelope(&self) -> f32 {
        SoundFontMath::cents_to_multiplying_factor(
            self.gs[GeneratorType::DECAY_VOLUME_ENVELOPE as usize] as f32,
        )
    }

    pub fn get_sustain_volume_envelope(&self) -> f32 {
        0.1_f32 * self.gs[GeneratorType::SUSTAIN_VOLUME_ENVELOPE as usize] as f32
    }

    pub fn get_release_volume_envelope(&self) -> f32 {
        SoundFontMath::cents_to_multiplying_factor(
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
        self.gs[GeneratorType::FINE_TUNE as usize] as i32
    }

    pub fn get_scale_tuning(&self) -> i32 {
        self.gs[GeneratorType::SCALE_TUNING as usize] as i32
    }
}
