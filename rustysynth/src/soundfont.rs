use std::error;
use std::io;
use std::rc;

use super::binary_reader;
use super::soundfont_info::SoundFontInfo;
use super::soundfont_sampledata::SoundFontSampleData;

#[non_exhaustive]
pub struct SoundFont
{
    pub info: SoundFontInfo,
    pub bits_per_sample: i32,
    pub wave_data: rc::Rc<Vec<i16>>,
}

impl SoundFont
{
    pub fn new<R: io::Read>(reader: &mut R) -> Result<Self, Box<dyn error::Error>>
    {
        let result = binary_reader::read_four_cc(reader);
        let chunk_id = match result
        {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        if chunk_id != "RIFF"
        {
            return Err(format!("The RIFF chunk was not found.").into());
        }

        let _size = binary_reader::read_i32(reader);

        let result = binary_reader::read_four_cc(reader);
        let form_type = match result
        {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        if form_type != "sfbk"
        {
            return Err(format!("The type of the RIFF chunk must be 'sfbk', but was '{form_type}'.").into());
        }

        let result = SoundFontInfo::new(reader);
        let info = match result
        {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        let result = SoundFontSampleData::new(reader);
        let wave_data = match result
        {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        Ok(SoundFont
        {
            info: info,
            bits_per_sample: 16,
            wave_data: wave_data.wave_data,
        })
    }
}
