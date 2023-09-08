#![allow(dead_code)]

use std::io::Read;

use crate::binary_reader::BinaryReader;
use crate::error::SoundFontError;
use crate::four_cc::FourCC;
use crate::generator::Generator;
use crate::instrument::Instrument;
use crate::instrument_info::InstrumentInfo;
use crate::preset::Preset;
use crate::preset_info::PresetInfo;
use crate::read_counter::ReadCounter;
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
    pub(crate) fn new<R: Read>(reader: &mut R) -> Result<Self, SoundFontError> {
        let chunk_id = BinaryReader::read_four_cc(reader)?;
        if chunk_id != b"LIST" {
            return Err(SoundFontError::ListChunkNotFound);
        }

        let end = BinaryReader::read_u32(reader)? as usize;
        let reader = &mut ReadCounter::new(reader);

        let list_type = BinaryReader::read_four_cc(reader)?;
        if list_type != b"pdta" {
            return Err(SoundFontError::InvalidListChunkType {
                expected: FourCC::from_bytes(*b"pdta"),
                actual: list_type,
            });
        }

        let mut preset_infos: Option<Vec<PresetInfo>> = None;
        let mut preset_bag: Option<Vec<ZoneInfo>> = None;
        let mut preset_generators: Option<Vec<Generator>> = None;
        let mut instrument_infos: Option<Vec<InstrumentInfo>> = None;
        let mut instrument_bag: Option<Vec<ZoneInfo>> = None;
        let mut instrument_generators: Option<Vec<Generator>> = None;
        let mut sample_headers: Option<Vec<SampleHeader>> = None;

        while reader.bytes_read() < end {
            let id = BinaryReader::read_four_cc(reader)?;
            let size = BinaryReader::read_u32(reader)? as usize;

            match id.as_bytes() {
                b"phdr" => preset_infos = Some(PresetInfo::read_from_chunk(reader, size)?),
                b"pbag" => preset_bag = Some(ZoneInfo::read_from_chunk(reader, size)?),
                b"pmod" => BinaryReader::discard_data(reader, size)?,
                b"pgen" => preset_generators = Some(Generator::read_from_chunk(reader, size)?),
                b"inst" => instrument_infos = Some(InstrumentInfo::read_from_chunk(reader, size)?),
                b"ibag" => instrument_bag = Some(ZoneInfo::read_from_chunk(reader, size)?),
                b"imod" => BinaryReader::discard_data(reader, size)?,
                b"igen" => instrument_generators = Some(Generator::read_from_chunk(reader, size)?),
                b"shdr" => sample_headers = Some(SampleHeader::read_from_chunk(reader, size)?),
                _ => return Err(SoundFontError::ListContainsUnknownId(id)),
            }
        }

        let preset_infos = preset_infos.ok_or(SoundFontError::SubChunkNotFound(
            FourCC::from_bytes(*b"PHDR"),
        ))?;

        let preset_bag = preset_bag.ok_or(SoundFontError::SubChunkNotFound(FourCC::from_bytes(
            *b"PBAG",
        )))?;

        let preset_generators = preset_generators.ok_or(SoundFontError::SubChunkNotFound(
            FourCC::from_bytes(*b"PGEN"),
        ))?;

        let instrument_infos = instrument_infos.ok_or(SoundFontError::SubChunkNotFound(
            FourCC::from_bytes(*b"INST"),
        ))?;

        let instrument_bag = instrument_bag.ok_or(SoundFontError::SubChunkNotFound(
            FourCC::from_bytes(*b"IBAG"),
        ))?;

        let instrument_generators = instrument_generators.ok_or(
            SoundFontError::SubChunkNotFound(FourCC::from_bytes(*b"IGEN")),
        )?;

        let sample_headers = sample_headers.ok_or(SoundFontError::SubChunkNotFound(
            FourCC::from_bytes(*b"SHDR"),
        ))?;

        let instrument_zones = Zone::create(&instrument_bag, &instrument_generators)?;
        let instruments =
            Instrument::create(&instrument_infos, &instrument_zones, &sample_headers)?;

        let preset_zones = Zone::create(&preset_bag, &preset_generators)?;
        let presets = Preset::create(&preset_infos, &preset_zones, &instruments)?;

        Ok(Self {
            sample_headers,
            presets,
            instruments,
        })
    }
}
