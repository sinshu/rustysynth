use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
#[non_exhaustive]
pub enum SynthesizerError {
    SampleRateOutOfRange(i32),
    BlockSizeOutOfRange(usize),
    MaximumPolyphonyOutOfRange(usize),
}

impl error::Error for SynthesizerError {}

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

#[derive(Debug)]
#[non_exhaustive]
pub enum SoundFontError {
    IoError(io::Error),
    RiffChunkNotFound,
    InvalidRiffChunkType {
        expected: &'static str,
        actual: String,
    },
    ListChunkNotFound,
    InvalidListChunkType {
        expected: &'static str,
        actual: String,
    },
    ListContainsUnknownId(String),
    SampleDataNotFound,
    UnsupportedSampleFormat,
    SubChunkNotFound(&'static str),
    InvalidPresetList,
    InvalidInstrumentId {
        preset_name: String,
        instrument_id: usize,
    },
    InvalidPreset(String),
    PresetNotFound,
    InvalidInstrumentList,
    InvalidSampleId {
        instrument_name: String,
        sample_id: usize,
    },
    InvalidInstrument(String),
    InstrumentNotFound,
    InvalidSampleHeaderList,
    InvalidZoneList,
    ZoneNotFound,
    InvalidGeneratorList,
}

impl error::Error for SoundFontError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SoundFontError::IoError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for SoundFontError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SoundFontError::IoError(err) => err.fmt(f),
            SoundFontError::RiffChunkNotFound => write!(f, "the RIFF chunk was not found"),
            SoundFontError::InvalidRiffChunkType { expected, actual } => write!(
                f,
                "the type of the RIFF chunk must be '{}', but was '{}'",
                expected, actual
            ),
            SoundFontError::ListChunkNotFound => write!(f, "the LIST chunk was not found"),
            SoundFontError::InvalidListChunkType { expected, actual } => write!(
                f,
                "the type of the LIST chunk must be '{}', but was '{}'",
                expected, actual
            ),
            SoundFontError::ListContainsUnknownId(id) => {
                write!(f, "the INFO list contains an unknown ID '{id}'")
            }
            SoundFontError::SampleDataNotFound => write!(f, "no valid sample data was found"),
            SoundFontError::UnsupportedSampleFormat => write!(f, "SoundFont3 is not yet supported"),
            SoundFontError::SubChunkNotFound(id) => {
                write!(f, "the '{}' sub-chunk was not found", id)
            }
            SoundFontError::InvalidPresetList => write!(f, "the preset list is invalid"),
            SoundFontError::InvalidInstrumentId {
                preset_name,
                instrument_id,
            } => write!(
                f,
                "the preset '{preset_name}' contains an invalid instrument ID '{instrument_id}'"
            ),
            SoundFontError::InvalidPreset(preset_name) => {
                write!(f, "the preset '{preset_name}' has no zone")
            }
            SoundFontError::PresetNotFound => write!(f, "no valid preset was found"),
            SoundFontError::InvalidInstrumentList => write!(f, "the instrument list is invalid"),
            SoundFontError::InvalidSampleId {
                instrument_name,
                sample_id,
            } => write!(
                f,
                "the instrument '{instrument_name}' contains an invalid sample ID '{sample_id}'"
            ),
            SoundFontError::InvalidInstrument(instrument_name) => {
                write!(f, "the instrument '{instrument_name}' has no zone")
            }
            SoundFontError::InstrumentNotFound => write!(f, "no valid instrument was found"),
            SoundFontError::InvalidSampleHeaderList => {
                write!(f, "the sample header list is invalid")
            }
            SoundFontError::InvalidZoneList => write!(f, "the zone list is invalid"),
            SoundFontError::ZoneNotFound => write!(f, "no valid zone was found"),
            SoundFontError::InvalidGeneratorList => write!(f, "the generator list is invalid"),
        }
    }
}

impl From<io::Error> for SoundFontError {
    fn from(err: io::Error) -> Self {
        SoundFontError::IoError(err)
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum MidiFileError {
    IoError(io::Error),
    InvalidChunkType {
        expected: &'static str,
        actual: String,
    },
    InvalidChunkData(&'static str),
    UnsupportedFormat(i16),
    InvalidTempoValue,
}

impl error::Error for MidiFileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MidiFileError::IoError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for MidiFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MidiFileError::IoError(err) => err.fmt(f),
            MidiFileError::InvalidChunkType { expected, actual } => write!(
                f,
                "the chunk type must be '{}', but was '{}'",
                expected, actual
            ),
            MidiFileError::InvalidChunkData(id) => write!(f, "the '{}' chunk has invalid data", id),
            MidiFileError::UnsupportedFormat(format) => {
                write!(f, "the format {} is not supported", format)
            }
            MidiFileError::InvalidTempoValue => write!(f, "failed to read the tempo value"),
        }
    }
}

impl From<io::Error> for MidiFileError {
    fn from(err: io::Error) -> Self {
        MidiFileError::IoError(err)
    }
}
