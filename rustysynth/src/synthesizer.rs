use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

use crate::SoundFont;
use crate::preset::Preset;
use crate::SynthesizerSettings;
use crate::voice_collection::VoiceCollection;
use crate::channel::Channel;

#[non_exhaustive]
pub struct Synthesizer
{
    pub sound_font: Rc<SoundFont>,
    pub sample_rate: i32,
    pub block_size: i32,
    pub maximum_polyphony: i32,
    pub enable_reverb_and_chorus: bool,

    pub(crate) minimum_voice_duration: i32,

    preset_lookup: HashMap<i32, usize>,
    default_preset: usize,

    pub(crate) channels: Vec<Channel>,

    voices: VoiceCollection,

    block_left: Vec<f32>,
    block_right: Vec<f32>,

    inverse_block_size: f32,

    block_read: usize,

    pub master_volume: f32,
}

impl Synthesizer
{
    pub const CHANNEL_COUNT: i32 = 16;
    pub const PERCUSSION_CHANNEL: i32 = 9;

    pub fn new(sound_font: &Rc<SoundFont>, settings: &SynthesizerSettings) -> Result<Self, Box<dyn Error>>
    {
        settings.validate()?;

        let minimum_voice_duration = settings.sample_rate / 500;

        let mut preset_lookup: HashMap<i32, usize> = HashMap::new();

        let mut min_preset_id = i32::MAX;
        let mut default_preset: usize = 0;
        for i in 0..sound_font.presets.len()
        {
            let preset = &sound_font.presets[i];

            // The preset ID is Int32, where the upper 16 bits represent the bank number
            // and the lower 16 bits represent the patch number.
            // This ID is used to search for presets by the combination of bank number
            // and patch number.
            let preset_id = (preset.bank_number << 16) | preset.patch_number;
            preset_lookup.insert(preset_id, i);

            // The preset with the minimum ID number will be default.
            // If the SoundFont is GM compatible, the piano will be chosen.
            if preset_id < min_preset_id
            {
                default_preset = i;
                min_preset_id = preset_id;
            }
        }

        let mut channels: Vec<Channel> = Vec::new();
        for i in 0..Synthesizer::CHANNEL_COUNT
        {
            channels.push(Channel::new(i == Synthesizer::PERCUSSION_CHANNEL));
        }

        let voices = VoiceCollection::new(settings);

        let block_left: Vec<f32> = vec![0_f32; settings.block_size as usize];
        let block_right: Vec<f32> = vec![0_f32; settings.block_size as usize];

        let inverse_block_size = 1_f32 / settings.block_size as f32;

        let block_read = settings.block_size as usize;

        let master_volume = 0.5_f32;

        Ok(Self
        {
            sound_font: Rc::clone(sound_font),
            sample_rate: settings.sample_rate,
            block_size: settings.block_size,
            maximum_polyphony: settings.maximum_polyphony,
            enable_reverb_and_chorus: settings.enable_reverb_and_chorus,
            minimum_voice_duration: minimum_voice_duration,
            preset_lookup: preset_lookup,
            default_preset: default_preset,
            channels: channels,
            voices: voices,
            block_left: block_left,
            block_right: block_right,
            inverse_block_size: inverse_block_size,
            block_read: block_read,
            master_volume: master_volume,
        })
    }
}
