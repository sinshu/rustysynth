#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum EnvelopeStage {
    Delay = 0,
    Attack = 1,
    Hold = 2,
    Decay = 3,
    Release = 4,
}

impl EnvelopeStage {
    pub const fn next(&self) -> Self {
        match self {
            Self::Delay => Self::Attack,
            Self::Attack => Self::Hold,
            Self::Hold => Self::Decay,
            Self::Decay => Self::Release,
            Self::Release => unreachable!(),
        }
    }
}
