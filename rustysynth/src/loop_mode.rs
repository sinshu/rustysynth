/// Specifies how the synthesizer loops the sample.
/// Corresponds to sampleModes enumerator in SoundFont 2 spec.
///
/// RustySynth doesn't support nonstandard loop modes.
/// Undefined values will fall back to `LoopMode::NoLoop`.
/// ```
/// use rustysynth::LoopMode;
///
/// assert_eq!(LoopMode::NoLoop as i16, 0);
/// assert_eq!(LoopMode::Continuous as u8, 1);
/// assert_eq!(LoopMode::LoopUntilNoteOff as u64 as f32, 3.0);
///
/// assert_eq!(LoopMode::from(0), LoopMode::NoLoop);
/// assert_eq!(LoopMode::from(1), LoopMode::Continuous);
/// assert_eq!(LoopMode::from(2), LoopMode::NoLoop);
/// assert_eq!(LoopMode::from(3), LoopMode::LoopUntilNoteOff);
/// assert_eq!(LoopMode::from(50), LoopMode::NoLoop);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum LoopMode {
    /// The sample will not loop.
    NoLoop = 0,
    /// The sample will loop continuously.
    Continuous = 1,
    /// The sample will loop and play the remainder once the note is released.
    LoopUntilNoteOff = 3,
}

impl From<i16> for LoopMode {
    fn from(value: i16) -> Self {
        match value {
            1 => LoopMode::Continuous,
            3 => LoopMode::LoopUntilNoteOff,
            _ => LoopMode::NoLoop,
        }
    }
}
