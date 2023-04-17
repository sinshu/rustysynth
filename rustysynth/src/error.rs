use std::fmt;

#[derive(Debug)]
pub enum SynthesizerError {
    SampleRateOutOfRange(i32),
    BlockSizeOutOfRange(usize),
    MaximumPolyphonyOutOfRange(usize),
}

impl fmt::Display for SynthesizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SynthesizerError::SampleRateOutOfRange(value) => write!(
                f,
                "the sample rate must be between 16000 and 192000, but was {}",
                value
            ),
            SynthesizerError::BlockSizeOutOfRange(value) => write!(
                f,
                "the block size must be between 8 and 1024, but was {}",
                value
            ),
            SynthesizerError::MaximumPolyphonyOutOfRange(value) => {
                write!(
                    f,
                    "the maximum number of polyphony must be between 8 and 256, but was {}",
                    value
                )
            }
        }
    }
}
