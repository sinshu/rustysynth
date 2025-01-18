#![allow(dead_code)]

use std::cmp;
use std::sync::Arc;

use crate::midifile::Message;
use crate::midifile::MidiFile;
use crate::synthesizer::Synthesizer;

/// An instance of the MIDI file sequencer.
#[derive(Debug)]
#[non_exhaustive]
pub struct MidiFileSequencer {
    synthesizer: Synthesizer,

    speed: f64,

    midi_file: Option<Arc<MidiFile>>,
    play_loop: bool,

    block_wrote: usize,

    current_time: f64,
    msg_index: usize,
    loop_index: usize,
}

impl MidiFileSequencer {
    /// Initializes a new instance of the sequencer.
    ///
    /// # Arguments
    ///
    /// * `synthesizer` - The synthesizer to be handled by the sequencer.
    pub fn new(synthesizer: Synthesizer) -> Self {
        Self {
            synthesizer,
            speed: 1.0,
            midi_file: None,
            play_loop: false,
            block_wrote: 0,
            current_time: 0.0,
            msg_index: 0,
            loop_index: 0,
        }
    }

    /// Plays the MIDI file.
    ///
    /// # Arguments
    ///
    /// * `midi_file` - The MIDI file to be played.
    /// * `play_loop` - If `true`, the MIDI file loops after reaching the end.
    pub fn play(&mut self, midi_file: &Arc<MidiFile>, play_loop: bool) {
        self.midi_file = Some(Arc::clone(midi_file));
        self.play_loop = play_loop;

        self.block_wrote = self.synthesizer.block_size;

        self.current_time = 0.0;
        self.msg_index = 0;
        self.loop_index = 0;

        self.synthesizer.reset()
    }

    /// Stops playing.
    pub fn stop(&mut self) {
        self.midi_file = None;
        self.synthesizer.reset();
    }

    /// Renders the waveform.
    ///
    /// # Arguments
    ///
    /// * `left` - The buffer of the left channel to store the rendered waveform.
    /// * `right` - The buffer of the right channel to store the rendered waveform.
    ///
    /// # Remarks
    ///
    /// The output buffers for the left and right must be the same length.
    pub fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        if left.len() != right.len() {
            panic!("The output buffers for the left and right must be the same length.");
        }

        let left_length = left.len();
        let mut wrote: usize = 0;
        while wrote < left_length {
            if self.block_wrote == self.synthesizer.block_size {
                self.process_events();
                self.block_wrote = 0;
                self.current_time += self.speed * self.synthesizer.block_size as f64
                    / self.synthesizer.sample_rate as f64;
            }

            let src_rem = self.synthesizer.block_size - self.block_wrote;
            let dst_rem = left_length - wrote;
            let rem = cmp::min(src_rem, dst_rem);

            self.synthesizer.render(
                &mut left[wrote..wrote + rem],
                &mut right[wrote..wrote + rem],
            );

            self.block_wrote += rem;
            wrote += rem;
        }
    }

    fn process_events(&mut self) {
        let midi_file = match self.midi_file.as_ref() {
            Some(value) => value,
            None => return,
        };

        while self.msg_index < midi_file.messages.len() {
            let time = midi_file.times[self.msg_index];
            let msg = midi_file.messages[self.msg_index];

            if time <= self.current_time {
                if msg.get_message_type() == Message::NORMAL {
                    self.synthesizer.process_midi_message(
                        msg.channel as i32,
                        msg.command as i32,
                        msg.data1 as i32,
                        msg.data2 as i32,
                    );
                } else if self.play_loop {
                    if msg.get_message_type() == Message::LOOP_START {
                        self.loop_index = self.msg_index;
                    } else if msg.get_message_type() == Message::LOOP_END {
                        self.current_time = midi_file.times[self.loop_index];
                        self.msg_index = self.loop_index;
                        self.synthesizer.note_off_all(false);
                    }
                }
                self.msg_index += 1;
            } else {
                break;
            }
        }

        if self.msg_index == midi_file.messages.len() && self.play_loop {
            self.current_time = midi_file.times[self.loop_index];
            self.msg_index = self.loop_index;
            self.synthesizer.note_off_all(false);
        }
    }

    /// Gets the synthesizer handled by the sequencer.
    pub fn get_synthesizer(&self) -> &Synthesizer {
        &self.synthesizer
    }

    /// Gets the currently playing MIDI file.
    pub fn get_midi_file(&self) -> Option<&MidiFile> {
        match &self.midi_file {
            None => None,
            Some(value) => Some(value),
        }
    }

    /// Gets the current playback position in seconds.
    pub fn get_position(&self) -> f64 {
        self.current_time
    }

    /// Gets a value that indicates whether the current playback position is at the end of the sequence.
    ///
    /// # Remarks
    ///
    /// If the `play` method has not yet been called, this value will be `true`.
    /// This value will never be `true` if loop playback is enabled.
    pub fn end_of_sequence(&self) -> bool {
        match &self.midi_file {
            None => true,
            Some(value) => self.msg_index == value.messages.len(),
        }
    }

    /// Gets the current playback speed.
    ///
    /// # Remarks
    ///
    /// The default value is 1.
    /// The tempo will be multiplied by this value during playback.
    pub fn get_speed(&self) -> f64 {
        self.speed
    }

    /// Sets the playback speed.
    ///
    /// # Remarks
    ///
    /// The value must be non-negative.
    pub fn set_speed(&mut self, value: f64) {
        if value < 0.0 {
            panic!("The playback speed must be a non-negative value.");
        }

        self.speed = value;
    }
}
