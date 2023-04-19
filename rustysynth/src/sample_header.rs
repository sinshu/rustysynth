#![allow(dead_code)]

use std::io::Read;

use crate::binary_reader::BinaryReader;
use crate::error::SoundFontError;

#[non_exhaustive]
pub struct SampleHeader {
    pub(crate) name: String,
    pub(crate) start: i32,
    pub(crate) end: i32,
    pub(crate) start_loop: i32,
    pub(crate) end_loop: i32,
    pub(crate) sample_rate: i32,
    pub(crate) original_pitch: u8,
    pub(crate) pitch_correction: i8,
    pub(crate) link: u16,
    pub(crate) sample_type: u16,
}

impl SampleHeader {
    fn new<R: Read>(reader: &mut R) -> Result<Self, SoundFontError> {
        let name = BinaryReader::read_fixed_length_string(reader, 20)?;
        let start = BinaryReader::read_i32(reader)?;
        let end = BinaryReader::read_i32(reader)?;
        let start_loop = BinaryReader::read_i32(reader)?;
        let end_loop = BinaryReader::read_i32(reader)?;
        let sample_rate = BinaryReader::read_i32(reader)?;
        let original_pitch = BinaryReader::read_u8(reader)?;
        let pitch_correction = BinaryReader::read_i8(reader)?;
        let link = BinaryReader::read_u16(reader)?;
        let sample_type = BinaryReader::read_u16(reader)?;

        Ok(Self {
            name,
            start,
            end,
            start_loop,
            end_loop,
            sample_rate,
            original_pitch,
            pitch_correction,
            link,
            sample_type,
        })
    }

    pub(crate) fn read_from_chunk<R: Read>(
        reader: &mut R,
        size: usize,
    ) -> Result<Vec<SampleHeader>, SoundFontError> {
        if size % 46 != 0 {
            return Err(SoundFontError::InvalidSampleHeaderList);
        }

        let count = size / 46 - 1;

        let mut headers: Vec<SampleHeader> = Vec::new();
        for _i in 0..count {
            headers.push(SampleHeader::new(reader)?);
        }

        // The last one is the terminator.
        SampleHeader::new(reader)?;

        Ok(headers)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_start(&self) -> i32 {
        self.start
    }

    pub fn get_end(&self) -> i32 {
        self.end
    }

    pub fn get_start_loop(&self) -> i32 {
        self.start_loop
    }

    pub fn get_end_loop(&self) -> i32 {
        self.end_loop
    }

    pub fn get_sample_rate(&self) -> i32 {
        self.sample_rate
    }

    pub fn get_original_pitch(&self) -> i32 {
        self.original_pitch as i32
    }

    pub fn get_pitch_correction(&self) -> i32 {
        self.pitch_correction as i32
    }

    pub fn get_link(&self) -> i32 {
        self.link as i32
    }

    pub fn get_sample_type(&self) -> i32 {
        self.sample_type as i32
    }
}
