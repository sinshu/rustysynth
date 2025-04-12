#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum EnvelopeStage {
    Delay = 0,
    Attack = 1,
    Hold = 2,
    Decay = 3,
    Release = 4,
}

impl EnvelopeStage {
    pub const fn next_stage(&self) -> Option<Self> {
        match self {
            Self::Delay => Some(Self::Attack),
            Self::Attack => Some(Self::Hold),
            Self::Hold => Some(Self::Decay),
            Self::Decay => Some(Self::Release),
            Self::Release => None,
        }
    }
}
