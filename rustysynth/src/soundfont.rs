#![allow(dead_code)]

use std::error::Error;
use std::io::Read;
use std::rc::Rc;

use crate::binary_reader::BinaryReader;
use crate::instrument::Instrument;
use crate::preset::Preset;
use crate::sample_header::SampleHeader;
use crate::soundfont_info::SoundFontInfo;
use crate::soundfont_parameters::SoundFontParameters;
use crate::soundfont_sampledata::SoundFontSampleData;

#[non_exhaustive]
pub struct SoundFont {
    pub(crate) info: SoundFontInfo,
    pub(crate) bits_per_sample: i32,
    pub(crate) wave_data: Rc<Vec<i16>>,
    pub(crate) sample_headers: Vec<SampleHeader>,
    pub(crate) presets: Vec<Preset>,
    pub(crate) instruments: Vec<Instrument>,
}

impl SoundFont {
    pub fn new<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let chunk_id = BinaryReader::read_four_cc(reader)?;
        if chunk_id != "RIFF" {
            return Err(format!("The RIFF chunk was not found.").into());
        }

        let _size = BinaryReader::read_i32(reader);

        let form_type = BinaryReader::read_four_cc(reader)?;
        if form_type != "sfbk" {
            return Err(format!(
                "The type of the RIFF chunk must be 'sfbk', but was '{form_type}'."
            )
            .into());
        }

        let info = SoundFontInfo::new(reader)?;
        let sample_data = SoundFontSampleData::new(reader)?;
        let parameters = SoundFontParameters::new(reader)?;

        Ok(Self {
            info: info,
            bits_per_sample: 16,
            wave_data: Rc::new(sample_data.wave_data),
            sample_headers: parameters.sample_headers,
            presets: parameters.presets,
            instruments: parameters.instruments,
        })
    }

    pub fn get_info(&self) -> &SoundFontInfo {
        &self.info
    }

    pub fn get_bits_per_sample(&self) -> i32 {
        self.bits_per_sample
    }

    pub fn get_wave_data(&self) -> &[i16] {
        &self.wave_data[..]
    }

    pub fn get_sample_headers(&self) -> &[SampleHeader] {
        &self.sample_headers[..]
    }

    pub fn get_presets(&self) -> &[Preset] {
        &self.presets[..]
    }

    pub fn get_instruments(&self) -> &[Instrument] {
        &self.instruments[..]
    }
}
