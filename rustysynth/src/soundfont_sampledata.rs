#![allow(dead_code)]

use std::io::Read;
use std::slice;

use crate::binary_reader::BinaryReader;
use crate::error::SoundFontError;
use crate::four_cc::FourCC;
use crate::read_counter::ReadCounter;

#[non_exhaustive]
pub struct SoundFontSampleData {
    pub bits_per_sample: i32,
    pub wave_data: Vec<i16>,
}

impl SoundFontSampleData {
    pub(crate) fn new<R: Read + ?Sized>(reader: &mut R) -> Result<Self, SoundFontError> {
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

        let wave_data = match wave_data {
            Some(value) => value,
            None => return Err(SoundFontError::SampleDataNotFound),
        };

        let ptr = wave_data.as_ptr() as *const u8;
        let four_cc = unsafe { slice::from_raw_parts(ptr, 4) };
        if four_cc == b"OggS" {
            return Err(SoundFontError::UnsupportedSampleFormat);
        }

        Ok(Self {
            bits_per_sample: 16,
            wave_data,
        })
    }
}
