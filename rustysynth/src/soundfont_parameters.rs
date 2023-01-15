#![allow(dead_code)]

use std::error::Error;
use std::io::Read;

use crate::binary_reader::BinaryReader;
use crate::generator::Generator;
use crate::instrument::Instrument;
use crate::instrument_info::InstrumentInfo;
use crate::preset::Preset;
use crate::preset_info::PresetInfo;
use crate::sample_header::SampleHeader;
use crate::zone::Zone;
use crate::zone_info::ZoneInfo;

#[non_exhaustive]
pub(crate) struct SoundFontParameters {
    pub(crate) sample_headers: Vec<SampleHeader>,
    pub(crate) presets: Vec<Preset>,
    pub(crate) instruments: Vec<Instrument>,
}

impl SoundFontParameters {
    pub(crate) fn new<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let chunk_id = BinaryReader::read_four_cc(reader)?;
        if chunk_id != "LIST" {
            return Err(format!("The LIST chunk was not found.").into());
        }

        let end = BinaryReader::read_i32(reader)?;

        let mut pos: i32 = 0;

        let list_type = BinaryReader::read_four_cc(reader)?;
        if list_type != "pdta" {
            return Err(format!(
                "The type of the LIST chunk must be 'pdta', but was '{list_type}'."
            )
            .into());
        }
        pos += 4;

        let mut preset_infos: Option<Vec<PresetInfo>> = None;
        let mut preset_bag: Option<Vec<ZoneInfo>> = None;
        let mut preset_generators: Option<Vec<Generator>> = None;
        let mut instrument_infos: Option<Vec<InstrumentInfo>> = None;
        let mut instrument_bag: Option<Vec<ZoneInfo>> = None;
        let mut instrument_generators: Option<Vec<Generator>> = None;
        let mut sample_headers: Option<Vec<SampleHeader>> = None;

        while pos < end {
            let id = BinaryReader::read_four_cc(reader)?;
            pos += 4;

            let size = BinaryReader::read_i32(reader)?;
            pos += 4;

            if id == "phdr" {
                preset_infos = Some(PresetInfo::read_from_chunk(reader, size)?);
            } else if id == "pbag" {
                preset_bag = Some(ZoneInfo::read_from_chunk(reader, size)?);
            } else if id == "pmod" {
                BinaryReader::discard_data(reader, size)?;
            } else if id == "pgen" {
                preset_generators = Some(Generator::read_from_chunk(reader, size)?);
            } else if id == "inst" {
                instrument_infos = Some(InstrumentInfo::read_from_chunk(reader, size)?);
            } else if id == "ibag" {
                instrument_bag = Some(ZoneInfo::read_from_chunk(reader, size)?);
            } else if id == "imod" {
                BinaryReader::discard_data(reader, size)?;
            } else if id == "igen" {
                instrument_generators = Some(Generator::read_from_chunk(reader, size)?);
            } else if id == "shdr" {
                sample_headers = Some(SampleHeader::read_from_chunk(reader, size)?);
            } else {
                return Err(format!("The INFO list contains an unknown ID '{id}'.").into());
            }

            pos += size;
        }

        let preset_infos = match preset_infos {
            Some(value) => value,
            None => return Err(format!("The PHDR sub-chunk was not found.").into()),
        };

        let preset_bag = match preset_bag {
            Some(value) => value,
            None => return Err(format!("The PBAG sub-chunk was not found.").into()),
        };

        let preset_generators = match preset_generators {
            Some(value) => value,
            None => return Err(format!("The PGEN sub-chunk was not found.").into()),
        };

        let instrument_infos = match instrument_infos {
            Some(value) => value,
            None => return Err(format!("The INST sub-chunk was not found.").into()),
        };

        let instrument_bag = match instrument_bag {
            Some(value) => value,
            None => return Err(format!("The IBAG sub-chunk was not found.").into()),
        };

        let instrument_generators = match instrument_generators {
            Some(value) => value,
            None => return Err(format!("The IGEN sub-chunk was not found.").into()),
        };

        let sample_headers = match sample_headers {
            Some(value) => value,
            None => return Err(format!("The SHDR sub-chunk was not found.").into()),
        };

        let instrument_zones = Zone::create(&instrument_bag, &instrument_generators)?;
        let instruments =
            Instrument::create(&instrument_infos, &instrument_zones, &sample_headers)?;

        let preset_zones = Zone::create(&preset_bag, &preset_generators)?;
        let presets = Preset::create(&preset_infos, &preset_zones, &instruments)?;

        Ok(Self {
            sample_headers: sample_headers,
            presets: presets,
            instruments: instruments,
        })
    }
}
