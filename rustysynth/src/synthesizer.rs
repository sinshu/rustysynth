use std::error::Error;
use std::rc::Rc;

use crate::SoundFont;
use crate::SynthesizerSettings;
use crate::channel::Channel;

#[non_exhaustive]
pub struct Synthesizer
{
    pub sound_font: Rc<SoundFont>,
    pub(crate) channels: Vec<Channel>,
    pub(crate) minimum_voice_duration: i32,
}

impl Synthesizer
{
    pub fn new(sound_font: Rc<SoundFont>, settings: SynthesizerSettings) -> Result<Self, Box<dyn Error>>
    {
        settings.validate()?;

        Ok(Self
        {
            sound_font: sound_font,
            channels: Vec::new(),
            minimum_voice_duration: 3,
        })
    }
}
