#![allow(dead_code)]

use std::io::Read;

use crate::binary_reader::BinaryReader;
use crate::disarmed_region::DisarmedRegion;
use crate::error::SoundFontError;
use crate::four_cc::FourCC;
use crate::generator_type::GeneratorType;
use crate::instrument::Instrument;
use crate::preset::Preset;
use crate::sample_header::SampleHeader;
use crate::soundfont_info::SoundFontInfo;
use crate::soundfont_parameters::SoundFontParameters;
use crate::soundfont_sampledata::SoundFontSampleData;
use crate::LoopMode;

/// Reperesents a SoundFont.
#[derive(Debug)]
#[non_exhaustive]
pub struct SoundFont {
    pub(crate) info: SoundFontInfo,
    pub(crate) bits_per_sample: i32,
    pub(crate) wave_data: Vec<i16>,
    pub(crate) sample_headers: Vec<SampleHeader>,
    pub(crate) presets: Vec<Preset>,
    pub(crate) instruments: Vec<Instrument>,
    pub(crate) disarmed_regions: Vec<DisarmedRegion>,
}

impl SoundFont {
    /// Loads a SoundFont from the stream.
    ///
    /// # Arguments
    ///
    /// * `reader` - The data stream used to load the SoundFont.
    pub fn new<R: Read>(reader: &mut R) -> Result<Self, SoundFontError> {
        let chunk_id = BinaryReader::read_four_cc(reader)?;
        if chunk_id != b"RIFF" {
            return Err(SoundFontError::RiffChunkNotFound);
        }

        let _size = BinaryReader::read_i32(reader)?;

        let form_type = BinaryReader::read_four_cc(reader)?;
        if form_type != b"sfbk" {
            return Err(SoundFontError::InvalidRiffChunkType {
                expected: FourCC::from_bytes(*b"sfbk"),
                actual: form_type,
            });
        }

        let info = SoundFontInfo::new(reader)?;
        let sample_data = SoundFontSampleData::new(reader)?;
        let parameters = SoundFontParameters::new(reader)?;

        let mut sound_font = Self {
            info,
            bits_per_sample: sample_data.bits_per_sample,
            wave_data: sample_data.wave_data,
            sample_headers: parameters.sample_headers,
            presets: parameters.presets,
            instruments: parameters.instruments,
            disarmed_regions: Vec::new(),
        };

        sound_font.disarmed_regions = sound_font.disarm_unplayable_regions();

        Ok(sound_font)
    }

    /// Makes every unplayable region harmless instead of refusing the file.
    ///
    /// The check this replaces returned `SanityCheckFailed` for the whole
    /// SoundFont over a single bad region. The check itself is needed -- see
    /// https://github.com/sinshu/rustysynth/issues/22,
    /// https://github.com/sinshu/rustysynth/issues/33 and
    /// https://github.com/sinshu/rustysynth/pull/51: `fill_block_continuous`
    /// wraps by subtracting `end_loop - start_loop`, so a negative loop length
    /// walks the read position out of the sample buffer. But several widely
    /// used SoundFonts carry a handful of such regions and play fine
    /// everywhere else (https://github.com/sinshu/rustysynth/issues/55), so
    /// the region is disarmed instead:
    ///
    /// - a region whose loop is unusable plays through once instead of looping;
    /// - a region that points outside the sample data is silenced.
    ///
    /// Loop points are judged only in regions that loop. A region that plays
    /// through once never reads them, so it is left exactly as it is and not
    /// reported.
    ///
    /// Returns every region it changed, so that the caller can tell which
    /// instruments no longer sound as their author made them.
    fn disarm_unplayable_regions(&mut self) -> Vec<DisarmedRegion> {
        let wave_len = self.wave_data.len() as i64;
        let mut disarmed = Vec::new();
        for (instrument_index, instrument) in self.instruments.iter_mut().enumerate() {
            for (region_index, region) in instrument.regions.iter_mut().enumerate() {
                // In i64: a header value plus a coarse address offset can pass
                // i32::MAX, which panics in a debug build.
                let start = region.sample_start as i64 + region.get_start_address_offset() as i64;
                let end = region.sample_end as i64 + region.get_end_address_offset() as i64;
                let start_loop =
                    region.sample_start_loop as i64 + region.get_start_loop_address_offset() as i64;
                let end_loop =
                    region.sample_end_loop as i64 + region.get_end_loop_address_offset() as i64;
                let looping = region.get_sample_modes() != LoopMode::NoLoop;

                // A sample rate of zero stalls the oscillator; one past 2^31 --
                // the field is unsigned in the file -- reads as negative and
                // runs it backwards out of the buffer.
                let unusable =
                    start < 0 || end >= wave_len || end <= start || region.sample_sample_rate <= 0;
                let loop_broken =
                    looping && (start_loop < 0 || end_loop >= wave_len || start_loop >= end_loop);

                if unusable {
                    // Nothing safe to play: the region stays silent. The address
                    // offsets go as well -- the bounds the oscillator gets are
                    // the header's plus these generators, so an end offset left
                    // in place would still point past the buffer.
                    region.gs[GeneratorType::SAMPLE_MODES as usize] = 0;
                    for gen in [
                        GeneratorType::START_ADDRESS_OFFSET,
                        GeneratorType::START_ADDRESS_COARSE_OFFSET,
                        GeneratorType::END_ADDRESS_OFFSET,
                        GeneratorType::END_ADDRESS_COARSE_OFFSET,
                        GeneratorType::START_LOOP_ADDRESS_OFFSET,
                        GeneratorType::START_LOOP_ADDRESS_COARSE_OFFSET,
                        GeneratorType::END_LOOP_ADDRESS_OFFSET,
                        GeneratorType::END_LOOP_ADDRESS_COARSE_OFFSET,
                    ] {
                        region.gs[gen as usize] = 0;
                    }
                    region.sample_start = 0;
                    region.sample_end = 0;
                    region.sample_start_loop = 0;
                    region.sample_end_loop = 0;
                } else if loop_broken {
                    // The sample is fine; only the loop is not. Play it through
                    // once instead of looping over it.
                    region.gs[GeneratorType::SAMPLE_MODES as usize] = 0;
                    region.sample_start_loop = region.sample_start;
                    region.sample_end_loop = region.sample_end;
                    for gen in [
                        GeneratorType::START_LOOP_ADDRESS_OFFSET,
                        GeneratorType::START_LOOP_ADDRESS_COARSE_OFFSET,
                        GeneratorType::END_LOOP_ADDRESS_OFFSET,
                        GeneratorType::END_LOOP_ADDRESS_COARSE_OFFSET,
                    ] {
                        region.gs[gen as usize] = 0;
                    }
                } else {
                    continue;
                }

                let sample_id = region.get_sample_id();
                disarmed.push(DisarmedRegion {
                    instrument_index,
                    region_index,
                    instrument: instrument.name.clone(),
                    sample: self
                        .sample_headers
                        .get(sample_id)
                        .map(|s| s.name.clone())
                        .unwrap_or_default(),
                    silenced: unusable,
                });
            }
        }
        disarmed
    }

    /// Gets the information of the SoundFont.
    pub fn get_info(&self) -> &SoundFontInfo {
        &self.info
    }

    /// Gets the bits per sample of the sample data.
    pub fn get_bits_per_sample(&self) -> i32 {
        self.bits_per_sample
    }

    /// Gets the sample data.
    pub fn get_wave_data(&self) -> &[i16] {
        &self.wave_data[..]
    }

    /// Gets the samples of the SoundFont.
    pub fn get_sample_headers(&self) -> &[SampleHeader] {
        &self.sample_headers[..]
    }

    /// Gets the presets of the SoundFont.
    pub fn get_presets(&self) -> &[Preset] {
        &self.presets[..]
    }

    /// Gets the instruments of the SoundFont.
    pub fn get_instruments(&self) -> &[Instrument] {
        &self.instruments[..]
    }

    /// Gets the regions the loader changed so that they could be played.
    ///
    /// A region whose loop is unusable plays through once instead of looping;
    /// a region that points outside the sample data is silenced. Empty for a
    /// SoundFont without such regions.
    pub fn get_disarmed_regions(&self) -> &[DisarmedRegion] {
        &self.disarmed_regions[..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fs::File, path::PathBuf};

    fn samples_dir_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("samples")
    }

    #[test]
    fn test_load_reject_sf3() {
        let path = samples_dir_path().join("dummy.sf3");
        let mut file = File::open(&path).unwrap();
        assert!(matches!(
            SoundFont::new(&mut file),
            Err(SoundFontError::UnsupportedSampleFormat)
        ));
    }

    // smpl sub-chunk exists, but is zero-length.
    #[test]
    fn test_load_empty_samples() {
        let path = samples_dir_path().join("test_empty_samples.sf2");
        let mut file = File::open(&path).unwrap();
        assert!(matches!(
            SoundFont::new(&mut file),
            Err(SoundFontError::SampleDataNotFound)
        ));
    }
}
