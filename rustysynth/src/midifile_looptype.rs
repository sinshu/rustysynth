/// Specifies the type of loop extension to use when playing back a MIDI file.
#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum MidiFileLoopType {
    /// Specifies the loop start point by a tick value.
    LoopPoint(usize),
    /// The RPG Maker style loop.
    /// CC #111 will be the loop start point.
    RpgMaker,
    /// The Incredible Machine style loop.
    /// CC #110 and #111 will be the start and end points of the loop.
    IncredibleMachine,
    /// The Final Fantasy style loop.
    /// CC #116 and #117 will be the start and end points of the loop.
    FinalFantasy,
}
