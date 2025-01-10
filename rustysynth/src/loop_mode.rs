#[derive(Debug, PartialEq, Eq)]
pub enum LoopMode {
    NoLoop,
    Continuous,
    LoopUntilNoteOff,
    Invalid(i16),
}

impl From<i16> for LoopMode {
    fn from(value: i16) -> Self {
        match value {
            0 => LoopMode::NoLoop,
            1 => LoopMode::Continuous,
            3 => LoopMode::LoopUntilNoteOff,
            _ => LoopMode::Invalid(value),
        }
    }
}

impl From<LoopMode> for i16 {
    fn from(mode: LoopMode) -> Self {
        match mode {
            LoopMode::NoLoop => 0,
            LoopMode::Continuous => 1,
            LoopMode::LoopUntilNoteOff => 3,
            LoopMode::Invalid(value) => value,
        }
    }
}
