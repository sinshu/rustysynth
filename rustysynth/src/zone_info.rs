use std::error;
use std::io;

use crate::binary_reader;

pub(crate) struct ZoneInfo
{
    pub(crate) generator_index: i32,
    pub(crate) modulator_index: i32,
    pub(crate) generator_count: i32,
    pub(crate) modulator_count: i32,
}

impl ZoneInfo
{
    fn new<R: io::Read>(reader: &mut R) -> Result<Self, Box<dyn error::Error>>
    {
        let result = binary_reader::read_u16(reader);
        let generator_index = match result
        {
            Ok(value) => value as i32,
            Err(error) => return Err(Box::new(error)),
        };

        let result = binary_reader::read_u16(reader);
        let modulator_index = match result
        {
            Ok(value) => value as i32,
            Err(error) => return Err(Box::new(error)),
        };

        Ok(ZoneInfo
        {
            generator_index: generator_index,
            modulator_index: modulator_index,
            generator_count: 0,
            modulator_count: 0,
        })
    }
}

pub(crate) fn read_from_chunk<R: io::Read>(reader: &mut R, size: i32) -> Result<Vec<ZoneInfo>, Box<dyn error::Error>>
{
    if size % 4 != 0
    {
        return Err(format!("The zone list is invalid.").into());
    }

    let count = size / 4;

    let mut zones: Vec<ZoneInfo> = Vec::new();
    for _i in 0..count
    {
        let result = ZoneInfo::new(reader);
        match result
        {
            Ok(value) => zones.push(value),
            Err(error) => return Err(error),
        }
    }

    for i in 0..(count - 1) as usize
    {
        zones[i].generator_count = zones[i + 1].generator_index - zones[i].generator_index;
        zones[i].modulator_count = zones[i + 1].modulator_index - zones[i].modulator_index;
    }

    Ok(zones)
}
