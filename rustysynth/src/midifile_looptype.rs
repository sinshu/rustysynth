#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum MidiFileLoopType {
    LoopPoint(usize),
    RpgMaker,
    IncredibleMachine,
    FinalFantasy,
}
