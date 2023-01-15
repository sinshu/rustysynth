#![allow(dead_code)]

#[allow(unused)]
#[non_exhaustive]
pub(crate) struct LoopMode {}

#[allow(unused)]
impl LoopMode {
    pub(crate) const NO_LOOP: i32 = 0;
    pub(crate) const CONTINUOUS: i32 = 0;
    pub(crate) const LOOP_UNTIL_NOTE_OFF: i32 = 0;
}
