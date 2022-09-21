use std::error;
use std::io;

use crate::binary_reader::BinaryReader;

pub struct SoundFontSampleData
{
    pub bits_per_sample: i32,
    pub wave_data: Vec<i16>,
}

impl SoundFontSampleData
{
    pub(crate) fn new<R: io::Read>(reader: &mut R) -> Result<Self, Box<dyn error::Error>>
    {
        let chunk_id = BinaryReader::read_four_cc(reader)?;
        if chunk_id != "LIST"
        {
            return Err(format!("The LIST chunk was not found.").into());
        }

        let end = BinaryReader::read_i32(reader)?;

        let mut pos: i32 = 0;

        let list_type = BinaryReader::read_four_cc(reader)?;
        if list_type != "sdta"
        {
            return Err(format!("The type of the LIST chunk must be 'sdta', but was '{list_type}'.").into());
        }
        pos += 4;

        let mut wave_data: Option<Vec<i16>> = None;

        while pos < end
        {
            let id = BinaryReader::read_four_cc(reader)?;
            pos += 4;

            let size = BinaryReader::read_i32(reader)?;
            pos += 4;

            if id == "smpl"
            {
                wave_data = Some(BinaryReader::read_wave_data(reader, size)?);
            }
            else if id == "sm24"
            {
                BinaryReader::discard_data(reader, size)?;
            }
            else
            {
                return Err(format!("The INFO list contains an unknown ID '{id}'.").into());
            }

            pos += size;
        }

        let wave_data = match wave_data
        {
            Some(value) => value,
            None => return Err(format!("No valid sample data was found.").into()),
        };

        Ok(SoundFontSampleData
        {
            bits_per_sample: 16,
            wave_data: wave_data,
        })
    }
}
