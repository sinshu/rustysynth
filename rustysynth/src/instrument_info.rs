#![allow(dead_code)]

use std::io::Read;

use crate::binary_reader::BinaryReader;
use crate::error::SoundFontError;

#[non_exhaustive]
pub(crate) struct InstrumentInfo {
    pub(crate) name: String,
    pub(crate) zone_start_index: i32,
    pub(crate) zone_end_index: i32,
}

impl InstrumentInfo {
    fn new<R: Read>(reader: &mut R) -> Result<Self, SoundFontError> {
        let name = BinaryReader::read_fixed_length_string(reader, 20)?;
        let zone_start_index = BinaryReader::read_u16(reader)? as i32;

        Ok(Self {
            name,
            zone_start_index,
            zone_end_index: 0,
        })
    }

    pub(crate) fn read_from_chunk<R: Read>(
        reader: &mut R,
        size: usize,
    ) -> Result<Vec<InstrumentInfo>, SoundFontError> {
        if size % 22 != 0 {
            return Err(SoundFontError::InvalidInstrumentList);
        }

        let count = size / 22;

        let mut instruments: Vec<InstrumentInfo> = Vec::new();
        for _i in 0..count {
            instruments.push(InstrumentInfo::new(reader)?);
        }

        for i in 0..(count - 1) {
            instruments[i].zone_end_index = instruments[i + 1].zone_start_index - 1;
        }

        Ok(instruments)
    }
}
