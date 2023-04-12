#![allow(dead_code)]
#![allow(unused_imports)]

use rustysynth::SampleHeader;

pub fn check(sample: &SampleHeader, values: &[i32; 7]) {
    assert_eq!(sample.get_start(), values[0]);
    assert_eq!(sample.get_end(), values[1]);
    assert_eq!(sample.get_start_loop(), values[2]);
    assert_eq!(sample.get_end_loop(), values[3]);
    assert_eq!(sample.get_sample_rate(), values[4]);
    assert_eq!(sample.get_original_pitch(), values[5]);
    assert_eq!(sample.get_pitch_correction(), values[6]);
}
