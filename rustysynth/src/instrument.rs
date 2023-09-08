#![allow(dead_code)]

use crate::error::SoundFontError;
use crate::instrument_info::InstrumentInfo;
use crate::instrument_region::InstrumentRegion;
use crate::sample_header::SampleHeader;
use crate::zone::Zone;

/// Represents an instrument in the SoundFont.
#[non_exhaustive]
pub struct Instrument {
    pub(crate) name: String,
    pub(crate) regions: Vec<InstrumentRegion>,
}

impl Instrument {
    fn new(
        info: &InstrumentInfo,
        instrument_id: usize,
        zones: &[Zone],
        samples: &[SampleHeader],
    ) -> Result<Self, SoundFontError> {
        let name = info.name.clone();

        let zone_count = info.zone_end_index - info.zone_start_index + 1;
        if zone_count <= 0 {
            return Err(SoundFontError::InvalidInstrument(instrument_id));
        }

        let span_start = info.zone_start_index as usize;
        let span_end = span_start + zone_count as usize;
        let zone_span = &zones[span_start..span_end];
        let regions = InstrumentRegion::create(instrument_id, zone_span, samples)?;

        Ok(Self { name, regions })
    }

    pub(crate) fn create(
        infos: &[InstrumentInfo],
        zones: &[Zone],
        samples: &[SampleHeader],
    ) -> Result<Vec<Instrument>, SoundFontError> {
        if infos.len() <= 1 {
            return Err(SoundFontError::InstrumentNotFound);
        }

        // The last one is the terminator.
        let count = infos.len() - 1;

        let mut instruments: Vec<Instrument> = Vec::new();
        for (instrument_id, info) in infos.iter().take(count).enumerate() {
            instruments.push(Instrument::new(info, instrument_id, zones, samples)?);
        }

        Ok(instruments)
    }

    /// Gets the name of the instrument.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Gets the regions of the instrument.
    pub fn get_regions(&self) -> &[InstrumentRegion] {
        &self.regions[..]
    }
}
