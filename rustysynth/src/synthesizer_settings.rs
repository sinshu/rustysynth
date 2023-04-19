#![allow(dead_code)]

use crate::error::SynthesizerError;

#[non_exhaustive]
pub struct SynthesizerSettings {
    pub sample_rate: i32,
    pub block_size: usize,
    pub maximum_polyphony: usize,
    pub enable_reverb_and_chorus: bool,
}

impl SynthesizerSettings {
    const DEFAULT_BLOCK_SIZE: usize = 64;
    const DEFAULT_MAXIMUM_POLYPHONY: usize = 64;
    const DEFAULT_ENABLE_REVERB_AND_CHORUS: bool = true;

    pub fn new(sample_rate: i32) -> Self {
        Self {
            sample_rate,
            block_size: SynthesizerSettings::DEFAULT_BLOCK_SIZE,
            maximum_polyphony: SynthesizerSettings::DEFAULT_MAXIMUM_POLYPHONY,
            enable_reverb_and_chorus: SynthesizerSettings::DEFAULT_ENABLE_REVERB_AND_CHORUS,
        }
    }

    pub(crate) fn validate(&self) -> Result<(), SynthesizerError> {
        SynthesizerSettings::check_sample_rate(self.sample_rate)?;
        SynthesizerSettings::check_block_size(self.block_size)?;
        SynthesizerSettings::check_maximum_polyphony(self.maximum_polyphony)?;

        Ok(())
    }

    fn check_sample_rate(value: i32) -> Result<(), SynthesizerError> {
        if !(16_000..=192_000).contains(&value) {
            return Err(SynthesizerError::SampleRateOutOfRange(value));
        }

        Ok(())
    }

    fn check_block_size(value: usize) -> Result<(), SynthesizerError> {
        if !(8..=1024).contains(&value) {
            return Err(SynthesizerError::BlockSizeOutOfRange(value));
        }

        Ok(())
    }

    fn check_maximum_polyphony(value: usize) -> Result<(), SynthesizerError> {
        if !(8..=256).contains(&value) {
            return Err(SynthesizerError::MaximumPolyphonyOutOfRange(value));
        }

        Ok(())
    }
}
