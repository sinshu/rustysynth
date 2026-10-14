#![allow(dead_code)]

use crate::error::SynthesizerError;

/// Specifies a set of parameters for synthesis.
#[derive(Debug)]
#[non_exhaustive]
pub struct SynthesizerSettings {
    /// The sample rate for synthesis.
    pub sample_rate: i32,
    /// The block size for rendering waveform.
    pub block_size: usize,
    /// The number of maximum polyphony, from [`MINIMUM_POLYPHONY`](Self::MINIMUM_POLYPHONY) to [`MAXIMUM_POLYPHONY`](Self::MAXIMUM_POLYPHONY);
    /// values outside this range make the synthesizer fail to be created. When all voices are in use, the lowest-priority voice is stolen for a new note.
    pub maximum_polyphony: usize,
    /// The number of voices allocated when the synthesizer is created (each takes under a kilobyte).
    /// More are allocated on demand, up to [`maximum_polyphony`](Self::maximum_polyphony) (a larger value is clamped to it).
    /// `None` (the default) allocates all of them up front, so nothing is allocated while rendering.
    /// A small value with a large `maximum_polyphony` costs nothing until notes need the room, if you can tolerate allocations.
    pub initial_polyphony: Option<usize>,
    /// The value indicating whether reverb and chorus are enabled.
    pub enable_reverb_and_chorus: bool,
}

impl SynthesizerSettings {
    const DEFAULT_BLOCK_SIZE: usize = 64;
    const DEFAULT_MAXIMUM_POLYPHONY: usize = 64;
    /// The fewest voices `maximum_polyphony` may be.
    pub const MINIMUM_POLYPHONY: usize = 8;
    /// The most voices `maximum_polyphony` may be. This only guards against nonsensical values, such as a negative number cast to an unsigned one;
    /// it is not a statement of what fits in memory (at under a kilobyte per voice, this many would take roughly 13 GB).
    pub const MAXIMUM_POLYPHONY: usize = 1 << 24;
    const DEFAULT_ENABLE_REVERB_AND_CHORUS: bool = true;

    /// Initializes a new instance of synthesizer settings.
    ///
    /// # Arguments
    ///
    /// * `sample_rate` - The sample rate for synthesis.
    pub fn new(sample_rate: i32) -> Self {
        Self {
            sample_rate,
            block_size: SynthesizerSettings::DEFAULT_BLOCK_SIZE,
            maximum_polyphony: SynthesizerSettings::DEFAULT_MAXIMUM_POLYPHONY,
            initial_polyphony: None,
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
        if !(SynthesizerSettings::MINIMUM_POLYPHONY..=SynthesizerSettings::MAXIMUM_POLYPHONY)
            .contains(&value)
        {
            return Err(SynthesizerError::MaximumPolyphonyOutOfRange(value));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polyphony_limits() {
        let mut settings = SynthesizerSettings::new(44100);
        for polyphony in [0, 7, SynthesizerSettings::MAXIMUM_POLYPHONY + 1, usize::MAX] {
            settings.maximum_polyphony = polyphony;
            assert!(settings.validate().is_err(), "{polyphony}");
        }
        for polyphony in [
            8,
            64,
            256,
            257,
            100_000,
            SynthesizerSettings::MAXIMUM_POLYPHONY,
        ] {
            settings.maximum_polyphony = polyphony;
            assert!(settings.validate().is_ok(), "{polyphony}");
        }
    }

    #[test]
    fn test_polyphony_error_message() {
        let message = SynthesizerError::MaximumPolyphonyOutOfRange(7).to_string();
        assert!(
            message.contains(&SynthesizerSettings::MINIMUM_POLYPHONY.to_string()),
            "{message}"
        );
        assert!(
            message.contains(&SynthesizerSettings::MAXIMUM_POLYPHONY.to_string()),
            "{message}"
        );
    }
}
