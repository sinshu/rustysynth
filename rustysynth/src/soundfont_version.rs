use std::io;

use super::binary_reader;

pub struct SoundFontVersion
{
    pub major: i16,
    pub minor: i16,

    _private: ()
}

impl SoundFontVersion
{
    pub(crate) fn default() -> Self
    {
        SoundFontVersion
        {
            major: 0,
            minor: 0,
            
            _private: (),
        }
    }

    pub(crate) fn new<R: io::Read>(reader: &mut R) -> Result<Self, io::Error>
    {
        let major = binary_reader::read_i16(reader)?;
        let minor = binary_reader::read_i16(reader)?;

        Ok(SoundFontVersion
        {
            major: major,
            minor: minor,

            _private: (),
        })
    }
}
