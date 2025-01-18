/// Specifies how the sample loops during playback.
#[derive(Debug, PartialEq, Eq)]
pub enum LoopMode {
    /// The sample will be played without loop.
    NoLoop,
    /// The sample will loop continuously.
    Continuous,
    /// The sample will loop until the note stops.
    LoopUntilNoteOff,
}

impl LoopMode {
    pub(crate) fn from_i16(value: i16) -> Self {
        match value {
            1 => LoopMode::Continuous,
            3 => LoopMode::LoopUntilNoteOff,
            _ => LoopMode::NoLoop,
        }
    }
}
