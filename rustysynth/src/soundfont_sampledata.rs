#![allow(dead_code)]

use std::io::Read;

use crate::binary_reader::BinaryReader;
use crate::error::SoundFontError;
use crate::four_cc::FourCC;
use crate::read_counter::ReadCounter;

#[non_exhaustive]
pub(crate) struct SoundFontSampleData {
    pub bits_per_sample: i32,
    pub wave_data: Vec<i16>,
}

impl SoundFontSampleData {
    pub(crate) fn new<R: Read>(reader: &mut R) -> Result<Self, SoundFontError> {
        let chunk_id = BinaryReader::read_four_cc(reader)?;
        if chunk_id != b"LIST" {
            return Err(SoundFontError::ListChunkNotFound);
        }

        let end = BinaryReader::read_u32(reader)? as usize;
        let reader = &mut ReadCounter::new(reader);

        let list_type = BinaryReader::read_four_cc(reader)?;
        if list_type != b"sdta" {
            return Err(SoundFontError::InvalidListChunkType {
                expected: FourCC::from_bytes(*b"sdta"),
                actual: list_type,
            });
        }

        let mut wave_data: Option<Vec<i16>> = None;

        while reader.bytes_read() < end {
            let id = BinaryReader::read_four_cc(reader)?;
            let size = BinaryReader::read_u32(reader)? as usize;

            match id.as_bytes() {
                b"smpl" => wave_data = Some(BinaryReader::read_wave_data(reader, size)?),
                b"sm24" => BinaryReader::discard_data(reader, size)?,
                _ => return Err(SoundFontError::ListContainsUnknownId(id)),
            }
        }

        let Some(wave_data) = wave_data else {
            return Err(SoundFontError::SampleDataNotFound);
        };

        if wave_data.len() < 2 {
            return Err(SoundFontError::SampleDataNotFound);
        }

        // SoundFont3 compressed sample format
        let four_cc = unsafe { std::slice::from_raw_parts(wave_data.as_ptr() as *const u8, 4) };
        if four_cc == b"OggS" {
            return Err(SoundFontError::UnsupportedSampleFormat);
        }

        Ok(Self {
            bits_per_sample: 16,
            wave_data,
        })
    }
}
