use std::error::Error;
use std::rc::Rc;

use crate::SoundFont;
use crate::SynthesizerSettings;

#[non_exhaustive]
pub struct Synthesizer
{
    pub sound_font: Rc<SoundFont>,
}

impl Synthesizer
{
    pub fn new(sound_font: Rc<SoundFont>, settings: SynthesizerSettings) -> Result<Self, Box<dyn Error>>
    {
        settings.validate()?;

        Ok(Self
        {
            sound_font: sound_font,
        })
    }
}
