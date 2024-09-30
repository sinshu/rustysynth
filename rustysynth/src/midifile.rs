#![allow(dead_code)]

use std::io::Read;

use crate::binary_reader::BinaryReader;
use crate::four_cc::FourCC;
use crate::read_counter::ReadCounter;
use crate::MidiFileError;
use crate::MidiFileLoopType;

#[derive(Clone, Copy)]
#[non_exhaustive]
pub(crate) struct Message {
    pub(crate) channel: u8,
    pub(crate) command: u8,
    pub(crate) data1: u8,
    pub(crate) data2: u8,
}

impl Message {
    pub(crate) const NORMAL: u8 = 0;
    pub(crate) const TEMPO_CHANGE: u8 = 252;
    pub(crate) const LOOP_START: u8 = 253;
    pub(crate) const LOOP_END: u8 = 254;
    pub(crate) const END_OF_TRACK: u8 = 255;

    pub(crate) fn common1(status: u8, data1: u8) -> Self {
        Self {
            channel: status & 0x0F,
            command: status & 0xF0,
            data1,
            data2: 0,
        }
    }

    pub(crate) fn common2(status: u8, data1: u8, data2: u8, loop_type: MidiFileLoopType) -> Self {
        let channel = status & 0x0F;
        let command = status & 0xF0;

        if command == 0xB0 {
            match loop_type {
                MidiFileLoopType::RpgMaker => {
                    if data1 == 111 {
                        return Message::loop_start();
                    }
                }

                MidiFileLoopType::IncredibleMachine => {
                    if data1 == 110 {
                        return Message::loop_start();
                    }
                    if data1 == 111 {
                        return Message::loop_end();
                    }
                }

                MidiFileLoopType::FinalFantasy => {
                    if data1 == 116 {
                        return Message::loop_start();
                    }
                    if data1 == 117 {
                        return Message::loop_end();
                    }
                }

                _ => (),
            }
        }

        Self {
            channel,
            command,
            data1,
            data2,
        }
    }

    pub(crate) fn tempo_change(tempo: i32) -> Self {
        Self {
            channel: Message::TEMPO_CHANGE,
            command: (tempo >> 16) as u8,
            data1: (tempo >> 8) as u8,
            data2: tempo as u8,
        }
    }

    pub(crate) fn loop_start() -> Self {
        Self {
            channel: Message::LOOP_START,
            command: 0,
            data1: 0,
            data2: 0,
        }
    }

    pub(crate) fn loop_end() -> Self {
        Self {
            channel: Message::LOOP_END,
            command: 0,
            data1: 0,
            data2: 0,
        }
    }

    pub(crate) fn end_of_track() -> Self {
        Self {
            channel: Message::END_OF_TRACK,
            command: 0,
            data1: 0,
            data2: 0,
        }
    }

    pub(crate) fn get_message_type(&self) -> u8 {
        match self.channel {
            Message::TEMPO_CHANGE => Message::TEMPO_CHANGE,
            Message::LOOP_START => Message::LOOP_START,
            Message::LOOP_END => Message::LOOP_END,
            Message::END_OF_TRACK => Message::END_OF_TRACK,
            _ => Message::NORMAL,
        }
    }

    pub(crate) fn get_tempo(&self) -> f64 {
        60000000.0
            / (((self.command as i32) << 16) | ((self.data1 as i32) << 8) | (self.data2 as i32))
                as f64
    }
}

/// Represents a standard MIDI file.
#[non_exhaustive]
pub struct MidiFile {
    pub(crate) messages: Vec<Message>,
    pub(crate) times: Vec<f64>,
}

impl MidiFile {
    /// Loads a MIDI file from the stream.
    ///
    /// # Arguments
    ///
    /// * `reader` - The data stream used to load the MIDI file.
    pub fn new<R: Read>(reader: &mut R) -> Result<Self, MidiFileError> {
        MidiFile::new_with_loop_type(reader, MidiFileLoopType::LoopPoint(0))
    }

    /// Loads a MIDI file from the stream with a specified loop type.
    ///
    /// # Arguments
    ///
    /// * `reader` - The data stream used to load the MIDI file.
    /// * `loop_type` - The type of the loop extension to be used.
    ///
    /// # Remarks
    ///
    /// `MidiFileLoopType` has the following variants:
    /// * `LoopPoint(usize)` - Specifies the loop start point by a tick value.
    /// * `RpgMaker` - The RPG Maker style loop.
    ///   CC #111 will be the loop start point.
    /// * `IncredibleMachine` - The Incredible Machine style loop.
    ///   CC #110 and #111 will be the start and end points of the loop.
    /// * `FinalFantasy` - The Final Fantasy style loop.
    ///   CC #116 and #117 will be the start and end points of the loop.
    pub fn new_with_loop_type<R: Read>(
        reader: &mut R,
        loop_type: MidiFileLoopType,
    ) -> Result<Self, MidiFileError> {
        let chunk_type = BinaryReader::read_four_cc(reader)?;
        if chunk_type != b"MThd" {
            return Err(MidiFileError::InvalidChunkType {
                expected: FourCC::from_bytes(*b"MThd"),
                actual: chunk_type,
            });
        }

        let size = BinaryReader::read_i32_big_endian(reader)?;
        if size != 6 {
            return Err(MidiFileError::InvalidChunkData(FourCC::from_bytes(
                *b"MThd",
            )));
        }

        let format = BinaryReader::read_i16_big_endian(reader)?;
        if !(format == 0 || format == 1) {
            return Err(MidiFileError::UnsupportedFormat(format));
        }

        let track_count = BinaryReader::read_i16_big_endian(reader)? as i32;
        let resolution = BinaryReader::read_i16_big_endian(reader)? as i32;

        let mut message_lists: Vec<Vec<Message>> = Vec::new();
        let mut tick_lists: Vec<Vec<i32>> = Vec::new();

        for _i in 0..track_count {
            let (message_list, tick_list) = MidiFile::read_track(reader, loop_type)?;
            message_lists.push(message_list);
            tick_lists.push(tick_list);
        }

        match loop_type {
            MidiFileLoopType::LoopPoint(loop_point) if loop_point != 0 => {
                let loop_point = loop_point as i32;
                let tick_list = &mut tick_lists[0];
                let message_list = &mut message_lists[0];

                if loop_point <= *tick_list.last().unwrap() {
                    for i in 0..tick_list.len() {
                        if tick_list[i] >= loop_point {
                            tick_list.insert(i, loop_point);
                            message_list.insert(i, Message::loop_start());
                            break;
                        }
                    }
                } else {
                    tick_list.push(loop_point);
                    message_list.push(Message::loop_start());
                }
            }
            _ => (),
        }

        let (messages, times) = MidiFile::merge_tracks(&message_lists, &tick_lists, resolution);

        Ok(Self { messages, times })
    }

    fn discard_data<R: Read>(reader: &mut R) -> Result<(), MidiFileError> {
        let size = BinaryReader::read_i32_variable_length(reader)? as usize;
        BinaryReader::discard_data(reader, size)?;
        Ok(())
    }

    fn read_tempo<R: Read>(reader: &mut R) -> Result<i32, MidiFileError> {
        let size = BinaryReader::read_i32_variable_length(reader)?;
        if size != 3 {
            return Err(MidiFileError::InvalidTempoValue);
        }

        let b1 = BinaryReader::read_u8(reader)? as i32;
        let b2 = BinaryReader::read_u8(reader)? as i32;
        let b3 = BinaryReader::read_u8(reader)? as i32;

        Ok((b1 << 16) | (b2 << 8) | b3)
    }

    fn read_track<R: Read>(
        reader: &mut R,
        loop_type: MidiFileLoopType,
    ) -> Result<(Vec<Message>, Vec<i32>), MidiFileError> {
        let chunk_type = BinaryReader::read_four_cc(reader)?;
        if chunk_type != b"MTrk" {
            return Err(MidiFileError::InvalidChunkType {
                expected: FourCC::from_bytes(*b"MTrk"),
                actual: chunk_type,
            });
        }

        let size = BinaryReader::read_i32_big_endian(reader)? as usize;
        let reader = &mut ReadCounter::new(reader);

        let mut messages: Vec<Message> = Vec::new();
        let mut ticks: Vec<i32> = Vec::new();

        let mut tick: i32 = 0;
        let mut last_status: u8 = 0;

        loop {
            let delta = BinaryReader::read_i32_variable_length(reader)?;
            let first = BinaryReader::read_u8(reader)?;

            tick += delta;

            if (first & 128) == 0 {
                let command = last_status & 0xF0;
                if command == 0xC0 || command == 0xD0 {
                    messages.push(Message::common1(last_status, first));
                    ticks.push(tick);
                } else {
                    let data2 = BinaryReader::read_u8(reader)?;
                    messages.push(Message::common2(last_status, first, data2, loop_type));
                    ticks.push(tick);
                }

                continue;
            }

            match first {
                0xF0 => MidiFile::discard_data(reader)?,
                0xF7 => MidiFile::discard_data(reader)?,
                0xFF => match BinaryReader::read_u8(reader)? {
                    0x2F => {
                        BinaryReader::read_u8(reader)?;
                        messages.push(Message::end_of_track());
                        ticks.push(tick);

                        // Some MIDI files may have events inserted after the EOT.
                        // Such events should be ignored.
                        if reader.bytes_read() < size {
                            BinaryReader::discard_data(reader, size - reader.bytes_read())?;
                        }

                        return Ok((messages, ticks));
                    }
                    0x51 => {
                        messages.push(Message::tempo_change(MidiFile::read_tempo(reader)?));
                        ticks.push(tick);
                    }
                    _ => MidiFile::discard_data(reader)?,
                },
                _ => {
                    let command = first & 0xF0;
                    if command == 0xC0 || command == 0xD0 {
                        let data1 = BinaryReader::read_u8(reader)?;
                        messages.push(Message::common1(first, data1));
                        ticks.push(tick);
                    } else {
                        let data1 = BinaryReader::read_u8(reader)?;
                        let data2 = BinaryReader::read_u8(reader)?;
                        messages.push(Message::common2(first, data1, data2, loop_type));
                        ticks.push(tick);
                    }
                }
            }

            last_status = first
        }
    }

    fn merge_tracks(
        message_lists: &[Vec<Message>],
        tick_lists: &[Vec<i32>],
        resolution: i32,
    ) -> (Vec<Message>, Vec<f64>) {
        let mut merged_messages: Vec<Message> = Vec::new();
        let mut merged_times: Vec<f64> = Vec::new();

        let mut indices: Vec<usize> = vec![0; message_lists.len()];

        let mut current_tick: i32 = 0;
        let mut current_time: f64 = 0.0;

        let mut tempo: f64 = 120.0;

        loop {
            let mut min_tick = i32::MAX;
            let mut min_index: i32 = -1;

            for ch in 0..tick_lists.len() {
                if indices[ch] < tick_lists[ch].len() {
                    let tick = tick_lists[ch][indices[ch]];
                    if tick < min_tick {
                        min_tick = tick;
                        min_index = ch as i32;
                    }
                }
            }

            if min_index == -1 {
                break;
            }

            let next_tick = tick_lists[min_index as usize][indices[min_index as usize]];
            let delta_tick = next_tick - current_tick;
            let delta_time = 60.0 / (resolution as f64 * tempo) * delta_tick as f64;

            current_tick += delta_tick;
            current_time += delta_time;

            let message = message_lists[min_index as usize][indices[min_index as usize]];
            if message.get_message_type() == Message::TEMPO_CHANGE {
                tempo = message.get_tempo();
            } else {
                merged_messages.push(message);
                merged_times.push(current_time);
            }

            indices[min_index as usize] += 1;
        }

        (merged_messages, merged_times)
    }

    /// Get the length of the MIDI file in seconds.
    pub fn get_length(&self) -> f64 {
        *self.times.last().unwrap()
    }
}
