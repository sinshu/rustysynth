use std::error;
use std::rc;

use crate::SoundFont;
use crate::SynthesizerSettings;

#[non_exhaustive]
pub struct Synthesizer
{
    pub sound_font: rc::Rc<SoundFont>,
}

impl Synthesizer
{
    pub fn new(sound_font: rc::Rc<SoundFont>, settings: SynthesizerSettings) -> Result<Self, Box<dyn error::Error>>
    {
        settings.validate()?;

        Ok(Synthesizer
        {
            sound_font: sound_font,
        })
    }
}
