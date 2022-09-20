use std::error;
use std::io;

use crate::binary_reader;

pub struct SampleHeader
{
    pub name: String,
    pub start: i32,
    pub end: i32,
    pub start_loop: i32,
    pub end_loop: i32,
    pub sample_rate: i32,
    pub original_pitch: u8,
    pub pitch_correction: i8,
    pub link: u16,
    pub sample_type: u16,
}

impl SampleHeader
{
    fn new<R: io::Read>(reader: &mut R) -> Result<Self, Box<dyn error::Error>>
    {
        let name = match binary_reader::read_fixed_length_string(reader, 20)
        {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        let start = match binary_reader::read_i32(reader)
        {
            Ok(value) => value,
            Err(error) => return Err(Box::new(error)),
        };

        let end = match binary_reader::read_i32(reader)
        {
            Ok(value) => value,
            Err(error) => return Err(Box::new(error)),
        };

        let start_loop = match binary_reader::read_i32(reader)
        {
            Ok(value) => value,
            Err(error) => return Err(Box::new(error)),
        };

        let end_loop = match binary_reader::read_i32(reader)
        {
            Ok(value) => value,
            Err(error) => return Err(Box::new(error)),
        };

        let sample_rate = match binary_reader::read_i32(reader)
        {
            Ok(value) => value,
            Err(error) => return Err(Box::new(error)),
        };

        let original_pitch = match binary_reader::read_u8(reader)
        {
            Ok(value) => value,
            Err(error) => return Err(Box::new(error)),
        };

        let pitch_correction = match binary_reader::read_i8(reader)
        {
            Ok(value) => value,
            Err(error) => return Err(Box::new(error)),
        };

        let link = match binary_reader::read_u16(reader)
        {
            Ok(value) => value,
            Err(error) => return Err(Box::new(error)),
        };

        let sample_type = match binary_reader::read_u16(reader)
        {
            Ok(value) => value,
            Err(error) => return Err(Box::new(error)),
        };

        Ok(SampleHeader
        {
            name: name,
            start: start,
            end: end,
            start_loop: start_loop,
            end_loop: end_loop,
            sample_rate: sample_rate,
            original_pitch: original_pitch,
            pitch_correction: pitch_correction,
            link: link,
            sample_type: sample_type,
        })
    }
}

pub(crate) fn read_from_chunk<R: io::Read>(reader: &mut R, size: i32) -> Result<Vec<SampleHeader>, Box<dyn error::Error>>
{
    if size % 46 != 0
    {
        return Err(format!("The sample header list is invalid.").into());
    }

    let count = size / 46 - 1;

    let mut headers: Vec<SampleHeader> = Vec::new();
    for _i in 0..count
    {
        match SampleHeader::new(reader)
        {
            Ok(value) => headers.push(value),
            Err(error) => return Err(error),
        }
    }

    // The last one is the terminator.
    match SampleHeader::new(reader)
    {
        Ok(_value) => (),
        Err(error) => return Err(error),
    };

    Ok(headers)
}
