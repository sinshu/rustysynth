#![allow(dead_code)]

use std::io::Read;
use std::slice;

use crate::binary_reader::BinaryReader;
use crate::error::SoundFontError;

#[non_exhaustive]
pub struct SoundFontSampleData {
    pub bits_per_sample: i32,
    pub wave_data: Vec<i16>,
}

impl SoundFontSampleData {
    pub(crate) fn new<R: Read>(reader: &mut R) -> Result<Self, SoundFontError> {
        let chunk_id = BinaryReader::read_four_cc(reader)?;
        if chunk_id != "LIST" {
            return Err(SoundFontError::ListChunkNotFound);
        }

        let end = BinaryReader::read_i32(reader)? as usize;

        let mut pos: usize = 0;

        let list_type = BinaryReader::read_four_cc(reader)?;
        if list_type != "sdta" {
            return Err(SoundFontError::InvalidListChunkType {
                expected: "sdta",
                actual: list_type,
            });
        }
        pos += 4;

        let mut wave_data: Option<Vec<i16>> = None;

        while pos < end {
            let id = BinaryReader::read_four_cc(reader)?;
            pos += 4;

            let size = BinaryReader::read_i32(reader)? as usize;
            pos += 4;

            if id == "smpl" {
                wave_data = Some(BinaryReader::read_wave_data(reader, size)?);
            } else if id == "sm24" {
                BinaryReader::discard_data(reader, size)?;
            } else {
                return Err(SoundFontError::ListContainsUnknownId(id));
            }

            pos += size;
        }

        let wave_data = match wave_data {
            Some(value) => value,
            None => return Err(SoundFontError::SampleDataNotFound),
        };

        let ptr = wave_data.as_ptr() as *const u8;
        let four_cc = unsafe { slice::from_raw_parts(ptr, 4) };
        if four_cc == "OggS".as_bytes() {
            return Err(SoundFontError::UnsupportedSampleFormat);
        }

        Ok(Self {
            bits_per_sample: 16,
            wave_data,
        })
    }
}
