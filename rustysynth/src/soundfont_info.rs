use std::error;
use std::io;

use super::binary_reader;
use super::soundfont_version::SoundFontVersion;

#[non_exhaustive]
pub struct SoundFontInfo
{
    pub version: SoundFontVersion,
    pub target_sound_engine: String,
    pub bank_name: String,
    pub rom_name: String,
    pub rom_version: SoundFontVersion,
    pub creation_date: String,
    pub auther: String,
    pub target_product: String,
    pub copyright: String,
    pub comments: String,
    pub tools: String,
}

impl SoundFontInfo
{
    pub(crate) fn new<R: io::Read>(reader: &mut R) -> Result<Self, Box<dyn error::Error>>
    {
        let result = binary_reader::read_four_cc(reader);
        let chunk_id = match result
        {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        if chunk_id != "LIST"
        {
            return Err(format!("The LIST chunk was not found.").into());
        }

        let result = binary_reader::read_i32(reader);
        let end = match result
        {
            Ok(value) => value,
            Err(error) => return Err(Box::new(error)),
        };

        let mut pos: i32 = 0;

        let result = binary_reader::read_four_cc(reader);
        let list_type = match result
        {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        if list_type != "INFO"
        {
            return Err(format!("The type of the LIST chunk must be 'INFO', but was '{list_type}'.").into());
        }
        pos += 4;

        let mut version: Option<SoundFontVersion> = None;
        let mut target_sound_engine: Option<String> = None;
        let mut bank_name: Option<String> = None;
        let mut rom_name: Option<String> = None;
        let mut rom_version: Option<SoundFontVersion> = None;
        let mut creation_date: Option<String> = None;
        let mut auther: Option<String> = None;
        let mut target_product: Option<String> = None;
        let mut copyright: Option<String> = None;
        let mut comments: Option<String> = None;
        let mut tools: Option<String> = None;

        while pos < end
        {
            let result = binary_reader::read_four_cc(reader);
            let id = match result
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

            if id == "ifil"
            {
                let result = SoundFontVersion::new(reader);
                version = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(Box::new(error)),
                };
            }

            if id == "isng"
            {
                let result = binary_reader::read_fixed_length_string(reader, size);
                target_sound_engine = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }

            if id == "INAM"
            {
                let result = binary_reader::read_fixed_length_string(reader, size);
                bank_name = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }

            if id == "irom"
            {
                let result = binary_reader::read_fixed_length_string(reader, size);
                rom_name = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }

            if id == "iver"
            {
                let result = SoundFontVersion::new(reader);
                rom_version = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(Box::new(error)),
                };
            }

            if id == "ICRD"
            {
                let result = binary_reader::read_fixed_length_string(reader, size);
                creation_date = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }

            if id == "IENG"
            {
                let result = binary_reader::read_fixed_length_string(reader, size);
                auther = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }

            if id == "IPRD"
            {
                let result = binary_reader::read_fixed_length_string(reader, size);
                target_product = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }

            if id == "ICOP"
            {
                let result = binary_reader::read_fixed_length_string(reader, size);
                copyright = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }

            if id == "ICMT"
            {
                let result = binary_reader::read_fixed_length_string(reader, size);
                comments = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }

            if id == "ISFT"
            {
                let result = binary_reader::read_fixed_length_string(reader, size);
                tools = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }

            pos += size;
        }

        Ok(SoundFontInfo
        {
            version: match version
            {
                Some(value) => value,
                None => SoundFontVersion::default(),
            },

            target_sound_engine: match target_sound_engine
            {
                Some(value) => value,
                None => String::new(),
            },

            bank_name: match bank_name
            {
                Some(value) => value,
                None => String::new(),
            },

            rom_name: match rom_name
            {
                Some(value) => value,
                None => String::new(),
            },

            rom_version: match rom_version
            {
                Some(value) => value,
                None => SoundFontVersion::default(),
            },

            creation_date: match creation_date
            {
                Some(value) => value,
                None => String::new(),
            },

            auther: match auther
            {
                Some(value) => value,
                None => String::new(),
            },

            target_product: match target_product
            {
                Some(value) => value,
                None => String::new(),
            },

            copyright: match copyright
            {
                Some(value) => value,
                None => String::new(),
            },

            comments: match comments
            {
                Some(value) => value,
                None => String::new(),
            },

            tools: match tools
            {
                Some(value) => value,
                None => String::new(),
            },
        })
    }
}
