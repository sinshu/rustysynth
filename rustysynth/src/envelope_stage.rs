#![allow(dead_code)]

#[allow(unused)]
#[non_exhaustive]
pub(crate) struct EnvelopeStage {}

#[allow(unused)]
impl EnvelopeStage {
    pub(crate) const DELAY: i32 = 0;
    pub(crate) const ATTACK: i32 = 1;
    pub(crate) const HOLD: i32 = 2;
    pub(crate) const DECAY: i32 = 3;
    pub(crate) const RELEASE: i32 = 4;
}
