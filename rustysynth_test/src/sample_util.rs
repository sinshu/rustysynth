#![allow(dead_code)]
#![allow(unused_imports)]

use rustysynth::SampleHeader;

pub fn check(sample: &SampleHeader, values: &[i32; 7])
{
    assert_eq!(sample.start as i32, values[0]);
    assert_eq!(sample.end as i32, values[1]);
    assert_eq!(sample.start_loop as i32, values[2]);
    assert_eq!(sample.end_loop as i32, values[3]);
    assert_eq!(sample.sample_rate as i32, values[4]);
    assert_eq!(sample.original_pitch as i32, values[5]);
    assert_eq!(sample.pitch_correction as i32, values[6]);
}
