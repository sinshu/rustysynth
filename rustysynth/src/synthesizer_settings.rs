use std::error::Error;

#[non_exhaustive]
pub struct SynthesizerSettings
{
    pub sample_rate: i32,
    pub block_size: i32,
    pub maximum_polyphony: i32,
    pub enable_reverb_and_chorus: bool,
}

impl SynthesizerSettings
{
    const DEFAULT_BLOCK_SIZE: i32 = 64;
    const DEFAULT_MAXIMUM_POLYPHONY: i32 = 64;
    const DEFAULT_ENABLE_REVERB_AND_CHORUS: bool = true;

    pub fn new(sample_rate: i32) -> Self
    {
        SynthesizerSettings
        {
            sample_rate: sample_rate,
            block_size: SynthesizerSettings::DEFAULT_BLOCK_SIZE,
            maximum_polyphony: SynthesizerSettings::DEFAULT_MAXIMUM_POLYPHONY,
            enable_reverb_and_chorus: SynthesizerSettings::DEFAULT_ENABLE_REVERB_AND_CHORUS,
        }
    }

    pub(crate) fn validate(&self) -> Result<(), Box<dyn Error>>
    {
        SynthesizerSettings::check_sample_rate(self.sample_rate)?;
        SynthesizerSettings::check_block_size(self.block_size)?;
        SynthesizerSettings::check_maximum_polyphony(self.maximum_polyphony)?;
        
        Ok(())
    }

    fn check_sample_rate(value: i32) -> Result<(), Box<dyn Error>>
    {
        if !(16000 <= value && value <= 192000)
        {
            return Err(format!("The sample rate must be between 16000 and 192000.").into());
        }

        Ok(())
    }

    fn check_block_size(value: i32) -> Result<(), Box<dyn Error>>
    {
        if !(8 <= value && value <= 1024)
        {
            return Err(format!("The block size must be between 8 and 1024.").into());
        }

        Ok(())
    }

    fn check_maximum_polyphony(value: i32) -> Result<(), Box<dyn Error>>
    {
        if !(8 <= value && value <= 256)
        {
            return Err(format!("The maximum number of polyphony must be between 8 and 256.").into());
        }

        Ok(())
    }
}
