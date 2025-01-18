#![allow(dead_code)]

use crate::error::SoundFontError;
use crate::instrument::Instrument;
use crate::preset_info::PresetInfo;
use crate::preset_region::PresetRegion;
use crate::zone::Zone;

/// Represents a preset in the SoundFont.
#[derive(Debug)]
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
        preset_id: usize,
        zones: &[Zone],
        instruments: &[Instrument],
    ) -> Result<Self, SoundFontError> {
        let name = info.name.clone();

        let zone_count = info.zone_end_index - info.zone_start_index + 1;
        if zone_count <= 0 {
            return Err(SoundFontError::InvalidPreset(preset_id));
        }

        let span_start = info.zone_start_index as usize;
        let span_end = span_start + zone_count as usize;
        let zone_span = &zones[span_start..span_end];
        let regions = PresetRegion::create(preset_id, zone_span, instruments)?;

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
        for (preset_id, info) in infos.iter().take(count).enumerate() {
            presets.push(Preset::new(info, preset_id, zones, instruments)?);
        }

        Ok(presets)
    }

    /// Gets the name of the preset.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Gets the patch number of the preset.
    pub fn get_patch_number(&self) -> i32 {
        self.patch_number
    }

    /// Gets the bank number of the preset.
    pub fn get_bank_number(&self) -> i32 {
        self.bank_number
    }

    /// Gets the library info.
    pub fn get_library(&self) -> i32 {
        self.library
    }

    /// Gets the genre info.
    pub fn get_genre(&self) -> i32 {
        self.genre
    }

    /// Gets the morphology info.
    pub fn get_morphology(&self) -> i32 {
        self.morphology
    }

    /// Gets the regions of the preset.
    pub fn get_regions(&self) -> &[PresetRegion] {
        &self.regions[..]
    }
}
