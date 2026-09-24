/// An instrument region the loader changed so that it can be played safely.
///
/// Instead of refusing the whole SoundFont over such a region, the loader
/// changes only the region and lists it here, so that nobody has to guess why
/// one instrument sounds different. See `SoundFont::get_disarmed_regions`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct DisarmedRegion {
    pub(crate) instrument: String,
    pub(crate) sample: String,
    pub(crate) silenced: bool,
}

impl DisarmedRegion {
    /// Gets the name of the instrument the region belongs to.
    pub fn get_instrument_name(&self) -> &str {
        &self.instrument
    }

    /// Gets the name of the sample the region plays.
    pub fn get_sample_name(&self) -> &str {
        &self.sample
    }

    /// True when the region points outside the sample data and was silenced;
    /// false when only its loop was unplayable and it now plays through once.
    pub fn is_silenced(&self) -> bool {
        self.silenced
    }
}
