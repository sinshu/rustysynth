#![allow(dead_code)]

use std::cmp;
use std::rc::Rc;

use crate::synthesizer::Synthesizer;
use crate::midifile::MidiFile;
use crate::midifile::Message;

#[non_exhaustive]
pub struct MidiFileSequencer
{
    midi_file: Option<Rc<MidiFile>>,
    play_loop: bool,

    block_wrote: usize,

    current_time: f64,
    msg_index: usize,

    block_left: Vec<f32>,
    block_right: Vec<f32>,
}

impl MidiFileSequencer
{
    pub fn new(synthesizer: &Synthesizer) -> Self
    {
        Self
        {
            midi_file: None,
            play_loop: false,
            block_wrote: 0,
            current_time: 0.0,
            msg_index: 0,
            block_left: vec![0_f32; synthesizer.block_size as usize],
            block_right: vec![0_f32; synthesizer.block_size as usize],
        }
    }

    pub fn play(&mut self, synthesizer: &mut Synthesizer, midi_file: &Rc<MidiFile>, play_loop: bool)
    {
        self.midi_file = Some(Rc::clone(midi_file));
        self.play_loop = play_loop;

        self.block_wrote = synthesizer.block_size as usize;

        self.current_time = 0.0;
        self.msg_index = 0;

        synthesizer.reset()
    }

    pub fn stop(&mut self, synthesizer: &mut Synthesizer)
    {
        self.midi_file = None;
        synthesizer.reset();
    }

    pub fn render(&mut self, synthesizer: &mut Synthesizer, left: &mut[f32], right: &mut[f32])
    {
        if left.len() != right.len()
        {
            panic!("The output buffers for the left and right must be the same length.");
        }

        let left_length = left.len();
        let mut wrote: usize = 0;
        while wrote < left_length
        {
            if self.block_wrote == synthesizer.block_size as usize
            {
                self.process_events(synthesizer);
                self.block_wrote = 0;
                self.current_time += synthesizer.block_size as f64 / synthesizer.sample_rate as f64;
            }

            let src_rem = synthesizer.block_size as usize - self.block_wrote;
            let dst_rem = left_length - wrote;
            let rem = cmp::min(src_rem, dst_rem);

            synthesizer.render(&mut left[wrote..wrote + rem], &mut right[wrote..wrote + rem]);

            self.block_wrote += rem;
            wrote += rem;
        }
    }

    fn process_events(&mut self, synthesizer: &mut Synthesizer)
    {
        let midi_file = match self.midi_file.as_ref()
        {
            Some(value) => value,
            None => return,
        };

        while self.msg_index < midi_file.messages.len()
        {
            let time = midi_file.times[self.msg_index];
            let msg = midi_file.messages[self.msg_index];

            if time <= self.current_time
            {
                if msg.get_message_type() == Message::NORMAL
                {
                    synthesizer.process_midi_message(msg.channel as i32, msg.command as i32, msg.data1 as i32, msg.data2 as i32);
                }
                self.msg_index += 1;
            }
            else
            {
                break;
            }
        }

        if self.msg_index == midi_file.messages.len() && self.play_loop
        {
            self.current_time = 0.0;
            self.msg_index = 0;
            synthesizer.note_off_all(false);
        }
    }
}
