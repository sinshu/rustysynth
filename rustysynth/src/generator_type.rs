#![allow(dead_code)]

#[allow(unused)]
#[non_exhaustive]
pub(crate) struct GeneratorType {}

#[allow(unused)]
impl GeneratorType {
    pub(crate) const START_ADDRESS_OFFSET: u16 = 0;
    pub(crate) const END_ADDRESS_OFFSET: u16 = 1;
    pub(crate) const START_LOOP_ADDRESS_OFFSET: u16 = 2;
    pub(crate) const END_LOOP_ADDRESS_OFFSET: u16 = 3;
    pub(crate) const START_ADDRESS_COARSE_OFFSET: u16 = 4;
    pub(crate) const MODULATION_LFO_TO_PITCH: u16 = 5;
    pub(crate) const VIBRATO_LFO_TO_PITCH: u16 = 6;
    pub(crate) const MODULATION_ENVELOPE_TO_PITCH: u16 = 7;
    pub(crate) const INITIAL_FILTER_CUTOFF_FREQUENCY: u16 = 8;
    pub(crate) const INITIAL_FILTER_Q: u16 = 9;
    pub(crate) const MODULATION_LFO_TO_FILTER_CUTOFF_FREQUENCY: u16 = 10;
    pub(crate) const MODULATION_ENVELOPE_TO_FILTER_CUTOFF_FREQUENCY: u16 = 11;
    pub(crate) const END_ADDRESS_COARSE_OFFSET: u16 = 12;
    pub(crate) const MODULATION_LFO_TO_VOLUME: u16 = 13;
    pub(crate) const UNUSED_1: u16 = 14;
    pub(crate) const CHORUS_EFFECTS_SEND: u16 = 15;
    pub(crate) const REVERB_EFFECTS_SEND: u16 = 16;
    pub(crate) const PAN: u16 = 17;
    pub(crate) const UNUSED_2: u16 = 18;
    pub(crate) const UNUSED_3: u16 = 19;
    pub(crate) const UNUSED_4: u16 = 20;
    pub(crate) const DELAY_MODULATION_LFO: u16 = 21;
    pub(crate) const FREQUENCY_MODULATION_LFO: u16 = 22;
    pub(crate) const DELAY_VIBRATO_LFO: u16 = 23;
    pub(crate) const FREQUENCY_VIBRATO_LFO: u16 = 24;
    pub(crate) const DELAY_MODULATION_ENVELOPE: u16 = 25;
    pub(crate) const ATTACK_MODULATION_ENVELOPE: u16 = 26;
    pub(crate) const HOLD_MODULATION_ENVELOPE: u16 = 27;
    pub(crate) const DECAY_MODULATION_ENVELOPE: u16 = 28;
    pub(crate) const SUSTAIN_MODULATION_ENVELOPE: u16 = 29;
    pub(crate) const RELEASE_MODULATION_ENVELOPE: u16 = 30;
    pub(crate) const KEY_NUMBER_TO_MODULATION_ENVELOPE_HOLD: u16 = 31;
    pub(crate) const KEY_NUMBER_TO_MODULATION_ENVELOPE_DECAY: u16 = 32;
    pub(crate) const DELAY_VOLUME_ENVELOPE: u16 = 33;
    pub(crate) const ATTACK_VOLUME_ENVELOPE: u16 = 34;
    pub(crate) const HOLD_VOLUME_ENVELOPE: u16 = 35;
    pub(crate) const DECAY_VOLUME_ENVELOPE: u16 = 36;
    pub(crate) const SUSTAIN_VOLUME_ENVELOPE: u16 = 37;
    pub(crate) const RELEASE_VOLUME_ENVELOPE: u16 = 38;
    pub(crate) const KEY_NUMBER_TO_VOLUME_ENVELOPE_HOLD: u16 = 39;
    pub(crate) const KEY_NUMBER_TO_VOLUME_ENVELOPE_DECAY: u16 = 40;
    pub(crate) const INSTRUMENT: u16 = 41;
    pub(crate) const RESERVED_1: u16 = 42;
    pub(crate) const KEY_RANGE: u16 = 43;
    pub(crate) const VELOCITY_RANGE: u16 = 44;
    pub(crate) const START_LOOP_ADDRESS_COARSE_OFFSET: u16 = 45;
    pub(crate) const KEY_NUMBER: u16 = 46;
    pub(crate) const VELOCITY: u16 = 47;
    pub(crate) const INITIAL_ATTENUATION: u16 = 48;
    pub(crate) const RESERVED_2: u16 = 49;
    pub(crate) const END_LOOP_ADDRESS_COARSE_OFFSET: u16 = 50;
    pub(crate) const COARSE_TUNE: u16 = 51;
    pub(crate) const FINE_TUNE: u16 = 52;
    pub(crate) const SAMPLE_ID: u16 = 53;
    pub(crate) const SAMPLE_MODES: u16 = 54;
    pub(crate) const RESERVED_3: u16 = 55;
    pub(crate) const SCALE_TUNING: u16 = 56;
    pub(crate) const EXCLUSIVE_CLASS: u16 = 57;
    pub(crate) const OVERRIDING_ROOT_KEY: u16 = 58;
    pub(crate) const UNUSED_5: u16 = 59;
    pub(crate) const UNUSED_END: u16 = 60;

    pub(crate) const COUNT: usize = 61;
}
