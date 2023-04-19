#![allow(dead_code)]

use crate::error::SoundFontError;
use crate::instrument::Instrument;
use crate::preset_info::PresetInfo;
use crate::preset_region::PresetRegion;
use crate::zone::Zone;

#[non_exhaustive]
pub struct Preset {
    pub(crate) name: String,
    pub(crate) patch_number: i32,
    pub(crate) bank_number: i32,
    pub(crate) library: i32,
    pub(crate) genre: i32,
    pub(crate) morphology: i32,
    pub(crate) regions: Vec<PresetRegion>,
}

impl Preset {
    fn new(
        info: &PresetInfo,
        zones: &[Zone],
        instruments: &[Instrument],
    ) -> Result<Self, SoundFontError> {
        let name = info.name.clone();

        let zone_count = info.zone_end_index - info.zone_start_index + 1;
        if zone_count <= 0 {
            return Err(SoundFontError::InvalidPreset(name));
        }

        let span_start = info.zone_start_index as usize;
        let span_end = span_start + zone_count as usize;
        let zone_span = &zones[span_start..span_end];
        let regions = PresetRegion::create(&name, zone_span, instruments)?;

        Ok(Self {
            name,
            patch_number: info.patch_number,
            bank_number: info.bank_number,
            library: info.library,
            genre: info.genre,
            morphology: info.morphology,
            regions,
        })
    }

    pub(crate) fn create(
        infos: &[PresetInfo],
        zones: &[Zone],
        instruments: &[Instrument],
    ) -> Result<Vec<Preset>, SoundFontError> {
        if infos.len() <= 1 {
            return Err(SoundFontError::PresetNotFound);
        }

        // The last one is the terminator.
        let count = infos.len() - 1;

        let mut presets: Vec<Preset> = Vec::new();
        for info in infos.iter().take(count) {
            presets.push(Preset::new(info, zones, instruments)?);
        }

        Ok(presets)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_patch_number(&self) -> i32 {
        self.patch_number
    }

    pub fn get_bank_number(&self) -> i32 {
        self.bank_number
    }

    pub fn get_library(&self) -> i32 {
        self.library
    }

    pub fn get_genre(&self) -> i32 {
        self.genre
    }

    pub fn get_morphology(&self) -> i32 {
        self.morphology
    }

    pub fn get_regions(&self) -> &[PresetRegion] {
        &self.regions[..]
    }
}
