use std::error;
use std::io;

use crate::binary_reader::BinaryReader;

pub(crate) struct PresetInfo
{
    pub(crate) name: String,
    pub(crate) patch_number: i32,
    pub(crate) bank_number: i32,
    pub(crate) zone_start_index: i32,
    pub(crate) zone_end_index: i32,
    pub(crate) library: i32,
    pub(crate) genre: i32,
    pub(crate) morphology: i32,
}

impl PresetInfo
{
    fn new<R: io::Read>(reader: &mut R) -> Result<Self, Box<dyn error::Error>>
    {
        let name = BinaryReader::read_fixed_length_string(reader, 20)?;
        let patch_number = BinaryReader::read_u16(reader)? as i32;
        let bank_number = BinaryReader::read_u16(reader)? as i32;
        let zone_start_index = BinaryReader::read_u16(reader)? as i32;
        let library = BinaryReader::read_i32(reader)?;
        let genre = BinaryReader::read_i32(reader)?;
        let morphology = BinaryReader::read_i32(reader)?;

        Ok(PresetInfo
        {
            name: name,
            patch_number: patch_number,
            bank_number: bank_number,
            zone_start_index: zone_start_index,
            zone_end_index: 0,
            library: library,
            genre: genre,
            morphology: morphology,
        })
    }

    pub(crate) fn read_from_chunk<R: io::Read>(reader: &mut R, size: i32) -> Result<Vec<PresetInfo>, Box<dyn error::Error>>
    {
        if size % 38 != 0
        {
            return Err(format!("The preset list is invalid.").into());
        }

        let count = size / 38;

        let mut presets: Vec<PresetInfo> = Vec::new();
        for _i in 0..count
        {
            presets.push(PresetInfo::new(reader)?);
        }

        for i in 0..(count - 1) as usize
        {
            presets[i].zone_end_index = presets[i + 1].zone_start_index - 1;
        }

        Ok(presets)
    }
}
