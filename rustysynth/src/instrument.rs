use std::error::Error;

use crate::zone::Zone;
use crate::instrument_info::InstrumentInfo;
use crate::instrument_region::InstrumentRegion;
use crate::sample_header::SampleHeader;

#[non_exhaustive]
pub struct Instrument
{
    pub name: String,
    pub regions: Vec<InstrumentRegion>,
}

impl Instrument
{
    fn new(info: &InstrumentInfo, zones: &Vec<Zone>, samples: &Vec<SampleHeader>) -> Result<Self, Box<dyn Error>>
    {
        let name = info.name.clone();

        let zone_count = info.zone_end_index - info.zone_start_index + 1;
        if zone_count <= 0
        {
            return Err(format!("The instrument '{name}' has no zone.").into());
        }

        let span_start = info.zone_start_index as usize;
        let span_end = span_start + zone_count as usize;
        let zone_span = &zones[span_start..span_end];
        let regions = InstrumentRegion::create(&name, zone_span, &samples)?;

        Ok(Instrument
        {
            name: name,
            regions: regions,
        })
    }

    pub(crate) fn create(infos: &Vec<InstrumentInfo>, zones: &Vec<Zone>, samples: &Vec<SampleHeader>) -> Result<Vec<Instrument>, Box<dyn Error>>
    {
        if infos.len() <= 1
        {
            return Err(format!("No valid instrument was found.").into());
        }

        // The last one is the terminator.
        let count = infos.len() - 1;

        let mut instruments: Vec<Instrument> = Vec::new();
        for i in 0..count
        {
            instruments.push(Instrument::new(&infos[i], &zones, &samples)?);
        }

        Ok(instruments)
    }
}
