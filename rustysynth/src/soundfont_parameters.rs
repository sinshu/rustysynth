use std::error;
use std::io;

use crate::generator;
use crate::generator::Generator;
use crate::preset_info;
use crate::preset_info::PresetInfo;
use crate::instrument_info;
use crate::instrument_info::InstrumentInfo;
use crate::sample_header;
use crate::sample_header::SampleHeader;
use crate::zone_info;
use crate::zone_info::ZoneInfo;

use super::binary_reader;

pub(crate) struct SoundFontParameters
{
    test: i32,
}

impl SoundFontParameters
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

            if id == "phdr"
            {
                let result = preset_info::read_from_chunk(reader, size);
                preset_infos = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "pbag"
            {
                let result = zone_info::read_from_chunk(reader, size);
                preset_bag = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "pmod"
            {
                let result = binary_reader::discard_data(reader, size);
                match result
                {
                    Ok(()) => (),
                    Err(error) => return Err(Box::new(error)),
                };
            }
            else if id == "pgen"
            {
                let result = generator::read_from_chunk(reader, size);
                preset_generators = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "inst"
            {
                let result = instrument_info::read_from_chunk(reader, size);
                instrument_infos = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "ibag"
            {
                let result = zone_info::read_from_chunk(reader, size);
                instrument_bag = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "imod"
            {
                let result = binary_reader::discard_data(reader, size);
                match result
                {
                    Ok(()) => (),
                    Err(error) => return Err(Box::new(error)),
                };
            }
            else if id == "igen"
            {
                let result = generator::read_from_chunk(reader, size);
                instrument_generators = match result
                {
                    Ok(value) => Some(value),
                    Err(error) => return Err(error),
                };
            }
            else if id == "shdr"
            {
                let result = sample_header::read_from_chunk(reader, size);
                sample_headers = match result
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

        println!("SOUNDFONT_PARAMS_DONE!!");

        Ok(SoundFontParameters { test: 3 })
    }
}
