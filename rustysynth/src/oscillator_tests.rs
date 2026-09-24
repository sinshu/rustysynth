//! Tests for the looping oscillator: a loop shorter than the playback step
//! must wrap back into the loop instead of walking past the sample buffer.

use std::sync::Arc;

use crate::generator_type::GeneratorType as G;
use crate::test_support::{render, Font};
use crate::SoundFont;

fn looping_font(loop_start: u32, loop_end: u32) -> Arc<SoundFont> {
    let mut font = Font::one_zone(&[(G::SAMPLE_MODES, 1)]);
    font.samples[0].loop_start = loop_start;
    font.samples[0].loop_end = loop_end;
    Arc::new(SoundFont::new(&mut &font.build()[..]).unwrap())
}

fn peak(samples: &[f32]) -> f32 {
    samples.iter().fold(0_f32, |m, x| m.max(x.abs()))
}

#[test]
fn a_loop_shorter_than_the_step_stays_inside_the_buffer() {
    // One frame of loop, played two octaves up: four frames per output
    // sample. Reported in https://github.com/sinshu/rustysynth/pull/58.
    let font = looping_font(1000, 1001);
    assert!(font.get_disarmed_regions().is_empty());
    for key in [84, 108, 127] {
        let out = render(&font, (0, 0), &[key], 100, 200);
        assert!(out.iter().all(|x| x.is_finite()), "key {key}");
        assert!(peak(&out) > 0.0, "key {key} should still sound");
    }
}

#[test]
fn every_short_loop_length_stays_inside_the_buffer() {
    for length in 1..=32 {
        let font = looping_font(1000, 1000 + length);
        let out = render(&font, (0, 0), &[127], 100, 50);
        assert!(out.iter().all(|x| x.is_finite()), "loop of {length} frames");
    }
}
