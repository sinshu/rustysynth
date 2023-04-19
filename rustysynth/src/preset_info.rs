#![allow(dead_code)]

use std::io::Read;

use crate::binary_reader::BinaryReader;
use crate::error::SoundFontError;

#[non_exhaustive]
pub(crate) struct PresetInfo {
    pub(crate) name: String,
    pub(crate) patch_number: i32,
    pub(crate) bank_number: i32,
    pub(crate) zone_start_index: i32,
    pub(crate) zone_end_index: i32,
    pub(crate) library: i32,
    pub(crate) genre: i32,
    pub(crate) morphology: i32,
}

impl PresetInfo {
    fn new<R: Read>(reader: &mut R) -> Result<Self, SoundFontError> {
        let name = BinaryReader::read_fixed_length_string(reader, 20)?;
        let patch_number = BinaryReader::read_u16(reader)? as i32;
        let bank_number = BinaryReader::read_u16(reader)? as i32;
        let zone_start_index = BinaryReader::read_u16(reader)? as i32;
        let library = BinaryReader::read_i32(reader)?;
        let genre = BinaryReader::read_i32(reader)?;
        let morphology = BinaryReader::read_i32(reader)?;

        Ok(Self {
            name,
            patch_number,
            bank_number,
            zone_start_index,
            zone_end_index: 0,
            library,
            genre,
            morphology,
        })
    }

    pub(crate) fn read_from_chunk<R: Read>(
        reader: &mut R,
        size: usize,
    ) -> Result<Vec<PresetInfo>, SoundFontError> {
        if size % 38 != 0 {
            return Err(SoundFontError::InvalidPresetList);
        }

        let count = size / 38;

        let mut presets: Vec<PresetInfo> = Vec::new();
        for _i in 0..count {
            presets.push(PresetInfo::new(reader)?);
        }

        for i in 0..(count - 1) {
            presets[i].zone_end_index = presets[i + 1].zone_start_index - 1;
        }

        Ok(presets)
    }
}
