use std::error;
use std::io;

use crate::binary_reader;
use crate::generator;
use crate::generator::Generator;
use crate::zone_info;
use crate::zone_info::ZoneInfo;
use crate::preset_info;
use crate::preset_info::PresetInfo;
use crate::instrument_info;
use crate::instrument_info::InstrumentInfo;
use crate::sample_header;
use crate::sample_header::SampleHeader;

pub(crate) struct SoundFontParameters
{
    test: i32,
}

impl SoundFontParameters
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
        if list_type != "pdta"
        {
            return Err(format!("The type of the LIST chunk must be 'pdta', but was '{list_type}'.").into());
        }
        pos += 4;

        let mut preset_infos: Option<Vec<PresetInfo>> = None;
        let mut preset_bag: Option<Vec<ZoneInfo>> = None;
        let mut preset_generators: Option<Vec<Generator>> = None;
        let mut instrument_infos: Option<Vec<InstrumentInfo>> = None;
        let mut instrument_bag: Option<Vec<ZoneInfo>> = None;
        let mut instrument_generators: Option<Vec<Generator>> = None;
        let mut sample_headers: Option<Vec<SampleHeader>> = None;

        while pos < end
        {
            let id = match binary_reader::read_four_cc(reader)
            {
                Ok(value) => value,
                Err(error) => return Err(error),
            };
            pos += 4;

            let size = match binary_reader::read_i32(reader)
            {
                Ok(value) => value,
                Err(error) => return Err(Box::new(error)),
            };
            pos += 4;

            if id == "phdr"
            {
                preset_infos = match preset_info::read_from_chunk(reader, size)
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "pbag"
            {
                preset_bag = match zone_info::read_from_chunk(reader, size)
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "pmod"
            {
                match binary_reader::discard_data(reader, size)
                {
                    Ok(()) => (),
                    Err(error) => return Err(Box::new(error)),
                };
            }
            else if id == "pgen"
            {
                preset_generators = match generator::read_from_chunk(reader, size)
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "inst"
            {
                instrument_infos = match instrument_info::read_from_chunk(reader, size)
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "ibag"
            {
                instrument_bag = match zone_info::read_from_chunk(reader, size)
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "imod"
            {
                match binary_reader::discard_data(reader, size)
                {
                    Ok(()) => (),
                    Err(error) => return Err(Box::new(error)),
                };
            }
            else if id == "igen"
            {
                instrument_generators = match generator::read_from_chunk(reader, size)
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "shdr"
            {
                sample_headers = match sample_header::read_from_chunk(reader, size)
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else
            {
                return Err(format!("The INFO list contains an unknown ID '{id}'.").into());
            }

            pos += size;
        }

        let preset_infos = match preset_infos
        {
            Some(value) => value,
            None => return Err(format!("The PHDR sub-chunk was not found.").into()),
        };

        let preset_bag = match preset_bag
        {
            Some(value) => value,
            None => return Err(format!("The PBAG sub-chunk was not found.").into()),
        };

        let preset_generators = match preset_generators
        {
            Some(value) => value,
            None => return Err(format!("The PGEN sub-chunk was not found.").into()),
        };

        let instrument_infos = match instrument_infos
        {
            Some(value) => value,
            None => return Err(format!("The INST sub-chunk was not found.").into()),
        };

        let instrument_bag = match instrument_bag
        {
            Some(value) => value,
            None => return Err(format!("The IBAG sub-chunk was not found.").into()),
        };

        let instrument_generators = match instrument_generators
        {
            Some(value) => value,
            None => return Err(format!("The IGEN sub-chunk was not found.").into()),
        };

        let sample_headers = match sample_headers
        {
            Some(value) => value,
            None => return Err(format!("The SHDR sub-chunk was not found.").into()),
        };

        println!("SOUNDFONT_PARAMS_DONE!!");

        Ok(SoundFontParameters { test: 3 })
    }
}
