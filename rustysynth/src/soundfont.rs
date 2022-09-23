#![allow(dead_code)]

use std::error::Error;
use std::io::Read;
use std::rc::Rc;

use crate::binary_reader::BinaryReader;
use crate::soundfont_info::SoundFontInfo;
use crate::soundfont_sampledata::SoundFontSampleData;
use crate::soundfont_parameters::SoundFontParameters;
use crate::sample_header::SampleHeader;
use crate::preset::Preset;
use crate::instrument::Instrument;

#[non_exhaustive]
pub struct SoundFont
{
    pub info: SoundFontInfo,
    pub bits_per_sample: i32,
    pub wave_data: Rc<Vec<i16>>,
    pub sample_headers: Vec<SampleHeader>,
    pub presets: Vec<Preset>,
    pub instruments: Vec<Instrument>,
}

impl SoundFont
{
    pub fn new<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>>
    {
        let chunk_id = BinaryReader::read_four_cc(reader)?;
        if chunk_id != "RIFF"
        {
            return Err(format!("The RIFF chunk was not found.").into());
        }

        let _size = BinaryReader::read_i32(reader);

        let form_type = BinaryReader::read_four_cc(reader)?;
        if form_type != "sfbk"
        {
            return Err(format!("The type of the RIFF chunk must be 'sfbk', but was '{form_type}'.").into());
        }

        let info = SoundFontInfo::new(reader)?;
        let sample_data = SoundFontSampleData::new(reader)?;
        let parameters = SoundFontParameters::new(reader)?;

        Ok(Self
        {
            info: info,
            bits_per_sample: 16,
            wave_data: Rc::new(sample_data.wave_data),
            sample_headers: parameters.sample_headers,
            presets: parameters.presets,
            instruments: parameters.instruments,
        })
    }
}
