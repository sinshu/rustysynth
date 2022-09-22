#![allow(dead_code)]
#![allow(unused_imports)]

use rustysynth::InstrumentRegion;

fn are_equal(x: f64, y: f64) -> bool
{
    if x.floor() == x.ceil() && y.floor() == y.ceil()
    {
        return x == y;
    }

    let m = if x.abs() > y.abs() { x.abs() } else { y.abs() };
    let limit = m / 1000_f64;
    let delta = (x - y).abs();

    delta < limit
}

pub fn check(region: &InstrumentRegion, values: &[f64; 50])
{
    assert!(are_equal(region.get_sample_start() as f64, values[0]));
    assert!(are_equal(region.get_sample_end() as f64, values[1]));
    assert!(are_equal(region.get_sample_start_loop() as f64, values[2]));
    assert!(are_equal(region.get_sample_end_loop() as f64, values[3]));
    assert!(are_equal(region.get_start_address_offset() as f64, values[4]));
    assert!(are_equal(region.get_end_address_offset() as f64, values[5]));
    assert!(are_equal(region.get_start_loop_address_offset() as f64, values[6]));
    assert!(are_equal(region.get_end_loop_address_offset() as f64, values[7]));
    assert!(are_equal(region.get_modulation_lfo_to_pitch() as f64, values[8]));
    assert!(are_equal(region.get_vibrato_lfo_to_pitch() as f64, values[9]));
    assert!(are_equal(region.get_modulation_envelope_to_pitch() as f64, values[10]));
    assert!(are_equal(region.get_initial_filter_cutoff_frequency() as f64, values[11]));
    assert!(are_equal(region.get_initial_filter_q() as f64, values[12]));
    assert!(are_equal(region.get_modulation_lfo_to_filter_cutoff_frequency() as f64, values[13]));
    assert!(are_equal(region.get_modulation_envelope_to_filter_cutoff_frequency() as f64, values[14]));
    assert!(are_equal(region.get_modulation_lfo_to_volume() as f64, values[15]));
    assert!(are_equal(region.get_chorus_effects_send() as f64, values[16]));
    assert!(are_equal(region.get_reverb_effects_send() as f64, values[17]));
    assert!(are_equal(region.get_pan() as f64, values[18]));
    assert!(are_equal(region.get_delay_modulation_lfo() as f64, values[19]));
    assert!(are_equal(region.get_frequency_modulation_lfo() as f64, values[20]));
    assert!(are_equal(region.get_delay_vibrato_lfo() as f64, values[21]));
    assert!(are_equal(region.get_frequency_vibrato_lfo() as f64, values[22]));
    assert!(are_equal(region.get_delay_modulation_envelope() as f64, values[23]));
    assert!(are_equal(region.get_attack_modulation_envelope() as f64, values[24]));
    assert!(are_equal(region.get_hold_modulation_envelope() as f64, values[25]));
    assert!(are_equal(region.get_decay_modulation_envelope() as f64, values[26]));
    assert!(are_equal(region.get_sustain_modulation_envelope() as f64, values[27]));
    assert!(are_equal(region.get_release_modulation_envelope() as f64, values[28]));
    assert!(are_equal(region.get_key_number_to_modulation_envelope_hold() as f64, values[29]));
    assert!(are_equal(region.get_key_number_to_modulation_envelope_decay() as f64, values[30]));
    assert!(are_equal(region.get_delay_volume_envelope() as f64, values[31]));
    assert!(are_equal(region.get_attack_volume_envelope() as f64, values[32]));
    assert!(are_equal(region.get_hold_volume_envelope() as f64, values[33]));
    assert!(are_equal(region.get_decay_volume_envelope() as f64, values[34]));
    assert!(are_equal(region.get_sustain_volume_envelope() as f64, values[35]));
    assert!(are_equal(region.get_release_volume_envelope() as f64, values[36]));
    assert!(are_equal(region.get_key_number_to_volume_envelope_hold() as f64, values[37]));
    assert!(are_equal(region.get_key_number_to_volume_envelope_decay() as f64, values[38]));
    assert!(are_equal(region.get_key_range_start() as f64, values[39]));
    assert!(are_equal(region.get_key_range_end() as f64, values[40]));
    assert!(are_equal(region.get_velocity_range_start() as f64, values[41]));
    assert!(are_equal(region.get_velocity_range_end() as f64, values[42]));
    assert!(are_equal(region.get_initial_attenuation() as f64, values[43]));
    assert!(are_equal(region.get_coarse_tune() as f64, values[44]));
    assert!(are_equal(region.get_fine_tune() as f64, values[45]));
    assert!(are_equal(region.get_sample_modes() as f64, values[46]));
    assert!(are_equal(region.get_scale_tuning() as f64, values[47]));
    assert!(are_equal(region.get_exclusive_class() as f64, values[48]));
    assert!(are_equal(region.get_root_key() as f64, values[49]));
}
