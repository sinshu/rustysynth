use std::error;
use std::io;

use crate::binary_reader;

pub struct SoundFontSampleData
{
    pub bits_per_sample: i32,
    pub wave_data: Vec<i16>,
}

impl SoundFontSampleData
{
    pub(crate) fn new<R: io::Read>(reader: &mut R) -> Result<Self, Box<dyn error::Error>>
    {
        let chunk_id = match binary_reader::read_four_cc(reader)
        {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        if chunk_id != "LIST"
        {
            return Err(format!("The LIST chunk was not found.").into());
        }

        let end = match binary_reader::read_i32(reader)
        {
            Ok(value) => value,
            Err(error) => return Err(Box::new(error)),
        };

        let mut pos: i32 = 0;

        let list_type = match binary_reader::read_four_cc(reader)
        {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        if list_type != "sdta"
        {
            return Err(format!("The type of the LIST chunk must be 'sdta', but was '{list_type}'.").into());
        }
        pos += 4;

        let mut wave_data: Option<Vec<i16>> = None;

        while pos < end
        {
            let id = match binary_reader::read_four_cc(reader)
            {
                Ok(value) => value,
                Err(error) => return Err(error),
            };
            pos += 4;

            let result = binary_reader::read_i32(reader);
            let size = match result
            {
                Ok(value) => value,
                Err(error) => return Err(Box::new(error)),
            };
            pos += 4;

            if id == "smpl"
            {
                wave_data = match binary_reader::read_wave_data(reader, size)
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(Box::new(error)),
                };
            }
            else if id == "sm24"
            {
                match binary_reader::discard_data(reader, size)
                {
                    Ok(()) => (),
                    Err(error) => return Err(Box::new(error)),
                };
            }
            else
            {
                return Err(format!("The INFO list contains an unknown ID '{id}'.").into());
            }

            pos += size;
        }

        Ok(SoundFontSampleData
        {
            bits_per_sample: 16,

            wave_data: match wave_data
            {
                Some(value) => value,
                None => return Err(format!("No valid sample data was found.").into()),
            },
        })
    }
}
