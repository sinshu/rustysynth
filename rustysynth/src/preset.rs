use std::error;

use crate::zone::Zone;
use crate::preset_info::PresetInfo;
use crate::preset_region::PresetRegion;
use crate::instrument::Instrument;

pub struct Preset
{
    pub name: String,
    pub patch_number: i32,
    pub bank_number: i32,
    pub library: i32,
    pub genre: i32,
    pub morphology: i32,
    pub regions: Vec<PresetRegion>,
}

impl Preset
{
    fn new(info: &PresetInfo, zones: &Vec<Zone>, instruments: &Vec<Instrument>) -> Result<Self, Box<dyn error::Error>>
    {
        let name = info.name.clone();

        let zone_count = info.zone_end_index - info.zone_start_index + 1;
        if zone_count <= 0
        {
            return Err(format!("The preset '{name}' has no zone.").into());
        }

        let span_start = info.zone_start_index as usize;
        let span_end = span_start + zone_count as usize;
        let zone_span = &zones[span_start..span_end];
        let regions = PresetRegion::create(&name, zone_span, &instruments)?;

        Ok(Preset
        {
            name: name,
            patch_number: info.patch_number,
            bank_number: info.bank_number,
            library: info.library,
            genre: info.genre,
            morphology: info.morphology,
            regions: regions,
        })
    }

    pub(crate) fn create(infos: &Vec<PresetInfo>, zones: &Vec<Zone>, instruments: &Vec<Instrument>) -> Result<Vec<Preset>, Box<dyn error::Error>>
    {
        if infos.len() <= 1
        {
            return Err(format!("No valid preset was found.").into());
        }

        // The last one is the terminator.
        let count = infos.len() - 1;

        let mut presets: Vec<Preset> = Vec::new();
        for i in 0..count
        {
            presets.push(Preset::new(&infos[i], &zones, &instruments)?);
        }

        Ok(presets)
    }
}
