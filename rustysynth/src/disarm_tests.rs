//! Tests for `SoundFont::disarm_unplayable_regions` and
//! `SoundFont::get_disarmed_regions`: an unplayable region is changed or
//! silenced, and reported, instead of the whole SoundFont being refused.

use std::sync::Arc;

use crate::generator_type::GeneratorType as G;
use crate::test_support::{render, Font};
use crate::{InstrumentRegion, LoopMode, SoundFont};

fn load(font: &Font) -> SoundFont {
    SoundFont::new(&mut &font.build()[..]).unwrap()
}

fn peak(samples: &[f32]) -> f32 {
    samples.iter().fold(0_f32, |m, x| m.max(x.abs()))
}

fn finite(samples: &[f32]) -> bool {
    samples.iter().all(|x| x.is_finite())
}

fn first_region(font: &SoundFont) -> &InstrumentRegion {
    &font.get_instruments()[0].get_regions()[0]
}

#[test]
fn a_healthy_looping_region_is_left_alone() {
    let font = load(&Font::one_zone(&[(G::SAMPLE_MODES, 1)]));
    assert!(font.get_disarmed_regions().is_empty());
    let region = first_region(&font);
    assert_eq!(region.get_sample_modes(), LoopMode::Continuous);
    assert_eq!(
        (region.get_sample_start_loop(), region.get_sample_end_loop()),
        (1000, 3000)
    );
}

#[test]
fn a_crossed_loop_plays_through_once_and_is_reported() {
    // Loop end moved to before loop start: the fault in Timbres of Heaven.
    let font = load(&Font::one_zone(&[
        (G::START_LOOP_ADDRESS_OFFSET, 1500),
        (G::END_LOOP_ADDRESS_OFFSET, -1500),
        (G::SAMPLE_MODES, 1),
    ]));

    let [disarmed] = font.get_disarmed_regions() else {
        panic!(
            "expected one disarmed region, got {:?}",
            font.get_disarmed_regions()
        );
    };
    assert_eq!(disarmed.get_instrument_name(), "instrument");
    assert_eq!(disarmed.get_sample_name(), "tone");
    assert!(!disarmed.is_silenced());

    let region = first_region(&font);
    assert_eq!(region.get_sample_modes(), LoopMode::NoLoop);
    assert_eq!(
        (region.get_sample_start(), region.get_sample_end()),
        (0, 4000)
    );

    let out = render(&Arc::new(font), (0, 0), &[60], 100, 100);
    assert!(finite(&out));
    assert!(peak(&out) > 0.0, "the sample should still sound");
}

#[test]
fn a_zero_length_loop_that_claims_to_loop_is_reported() {
    let mut font = Font::one_zone(&[(G::SAMPLE_MODES, 1)]);
    font.samples[0].loop_start = 2000;
    font.samples[0].loop_end = 2000;
    let font = load(&font);
    assert_eq!(font.get_disarmed_regions().len(), 1);
    assert_eq!(first_region(&font).get_sample_modes(), LoopMode::NoLoop);
}

#[test]
fn a_region_that_does_not_loop_keeps_its_loop_points_and_is_not_reported() {
    // Crossed loop points, but no loop mode: the points are never read.
    let font = load(&Font::one_zone(&[
        (G::START_LOOP_ADDRESS_OFFSET, 1500),
        (G::END_LOOP_ADDRESS_OFFSET, -1500),
    ]));
    assert!(font.get_disarmed_regions().is_empty());
    let region = first_region(&font);
    assert_eq!(
        (region.get_sample_start_loop(), region.get_sample_end_loop()),
        (2500, 1500)
    );
}

#[test]
fn a_region_outside_the_sample_data_is_silenced_and_reported() {
    let font = load(&Font::one_zone(&[
        (G::END_ADDRESS_COARSE_OFFSET, 10),
        (G::SAMPLE_MODES, 1),
    ]));

    let [disarmed] = font.get_disarmed_regions() else {
        panic!("expected one disarmed region");
    };
    assert!(disarmed.is_silenced());
    let region = first_region(&font);
    assert_eq!((region.get_sample_start(), region.get_sample_end()), (0, 0));

    let out = render(&Arc::new(font), (0, 0), &[60], 100, 50);
    assert!(finite(&out));
    assert_eq!(peak(&out), 0.0);
}

#[test]
fn a_sample_rate_of_zero_or_past_i32_max_is_silenced() {
    for rate in [0, 0x8000_0000] {
        let mut font = Font::one_zone(&[(G::SAMPLE_MODES, 1)]);
        font.samples[0].rate = rate;
        let font = load(&font);
        assert!(font.get_disarmed_regions()[0].is_silenced(), "rate {rate}");
        let out = render(&Arc::new(font), (0, 0), &[60], 100, 20);
        assert!(finite(&out));
    }
}

#[test]
fn bounds_that_overflow_i32_are_silenced_not_wrapped() {
    let mut font = Font::one_zone(&[
        (G::END_ADDRESS_COARSE_OFFSET, i16::MAX),
        (G::SAMPLE_MODES, 1),
    ]);
    font.samples[0].raw_bounds = Some([0, 0x7FFF_FFF0, 10, 20]);
    let font = load(&font);
    assert!(font.get_disarmed_regions()[0].is_silenced());
    let out = render(&Arc::new(font), (0, 0), &[60], 100, 20);
    assert!(finite(&out));
}

#[test]
fn only_the_broken_region_changes() {
    let mut font = Font::several();
    // The high half of the key split gets a crossed loop; the low half keeps
    // the loop it inherits from the global zone.
    font.instruments[0].1[2].insert(0, (G::END_LOOP_ADDRESS_OFFSET, -2000));
    let font = load(&font);
    let found: Vec<_> = font
        .get_disarmed_regions()
        .iter()
        .map(|d| {
            (
                d.get_instrument_index(),
                d.get_region_index(),
                d.get_sample_name(),
            )
        })
        .collect();
    assert_eq!(found, [(0, 1, "high")]);
    let regions = font.get_instruments()[0].get_regions();
    assert_eq!(regions[0].get_sample_modes(), LoopMode::Continuous);
    assert_eq!(regions[1].get_sample_modes(), LoopMode::NoLoop);
}
