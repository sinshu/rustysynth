use crate::midifile::Message;

/// Specifies the type of the loop extension to use when playing back a MIDI file.
#[derive(Clone, Copy, Debug)]
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

impl MidiFileLoopType {
    /// Map proprietary loop messages to RustySynth internal format
    pub(crate) const fn map_message(&self, message: Message) -> Option<Message> {
        // Not a CC event
        if message.command != 0xB0 {
            return None;
        }

        match self {
            Self::LoopPoint(_) => None,
            Self::RpgMaker => match message.data1 {
                111 => Some(Message::loop_start()),
                _ => None,
            },
            Self::IncredibleMachine => match message.data1 {
                110 => Some(Message::loop_start()),
                111 => Some(Message::loop_end()),
                _ => None,
            },
            Self::FinalFantasy => match message.data1 {
                116 => Some(Message::loop_start()),
                117 => Some(Message::loop_end()),
                _ => None,
            },
        }
    }
}
