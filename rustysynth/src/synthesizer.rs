#![allow(dead_code)]

use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use crate::array_math::ArrayMath;
use crate::channel::Channel;
use crate::chorus::Chorus;
use crate::region_pair::RegionPair;
use crate::reverb::Reverb;
use crate::soundfont::SoundFont;
use crate::soundfont_math::SoundFontMath;
use crate::synthesizer_settings::SynthesizerSettings;
use crate::voice_collection::VoiceCollection;

#[non_exhaustive]
pub struct Synthesizer {
    pub(crate) sound_font: Arc<SoundFont>,
    pub(crate) sample_rate: i32,
    pub(crate) block_size: i32,
    pub(crate) maximum_polyphony: i32,
    pub(crate) enable_reverb_and_chorus: bool,

    minimum_voice_duration: i32,

    preset_lookup: HashMap<i32, usize>,
    default_preset: usize,

    channels: Vec<Channel>,

    voices: VoiceCollection,

    block_left: Vec<f32>,
    block_right: Vec<f32>,

    inverse_block_size: f32,

    block_read: usize,

    pub(crate) master_volume: f32,

    reverb: Option<Reverb>,
    reverb_input: Option<Vec<f32>>,
    reverb_output_left: Option<Vec<f32>>,
    reverb_output_right: Option<Vec<f32>>,

    chorus: Option<Chorus>,
    chorus_input_left: Option<Vec<f32>>,
    chorus_input_right: Option<Vec<f32>>,
    chorus_output_left: Option<Vec<f32>>,
    chorus_output_right: Option<Vec<f32>>,
}

impl Synthesizer {
    pub const CHANNEL_COUNT: i32 = 16;
    pub const PERCUSSION_CHANNEL: i32 = 9;

    pub fn new(
        sound_font: &Arc<SoundFont>,
        settings: &SynthesizerSettings,
    ) -> Result<Self, Box<dyn Error>> {
        settings.validate()?;

        let minimum_voice_duration = settings.sample_rate / 500;

        let mut preset_lookup: HashMap<i32, usize> = HashMap::new();

        let mut min_preset_id = i32::MAX;
        let mut default_preset: usize = 0;
        for i in 0..sound_font.presets.len() {
            let preset = &sound_font.presets[i];

            // The preset ID is Int32, where the upper 16 bits represent the bank number
            // and the lower 16 bits represent the patch number.
            // This ID is used to search for presets by the combination of bank number
            // and patch number.
            let preset_id = (preset.bank_number << 16) | preset.patch_number;
            preset_lookup.insert(preset_id, i);

            // The preset with the minimum ID number will be default.
            // If the SoundFont is GM compatible, the piano will be chosen.
            if preset_id < min_preset_id {
                default_preset = i;
                min_preset_id = preset_id;
            }
        }

        let mut channels: Vec<Channel> = Vec::new();
        for i in 0..Synthesizer::CHANNEL_COUNT {
            channels.push(Channel::new(i == Synthesizer::PERCUSSION_CHANNEL));
        }

        let voices = VoiceCollection::new(settings);

        let block_left: Vec<f32> = vec![0_f32; settings.block_size as usize];
        let block_right: Vec<f32> = vec![0_f32; settings.block_size as usize];

        let inverse_block_size = 1_f32 / settings.block_size as f32;

        let block_read = settings.block_size as usize;

        let master_volume = 0.5_f32;

        let reverb = if settings.enable_reverb_and_chorus {
            Some(Reverb::new(settings.sample_rate))
        } else {
            None
        };
        let reverb_input = if settings.enable_reverb_and_chorus {
            Some(vec![0_f32; settings.block_size as usize])
        } else {
            None
        };
        let reverb_output_left = if settings.enable_reverb_and_chorus {
            Some(vec![0_f32; settings.block_size as usize])
        } else {
            None
        };
        let reverb_output_right = if settings.enable_reverb_and_chorus {
            Some(vec![0_f32; settings.block_size as usize])
        } else {
            None
        };

        let chorus = if settings.enable_reverb_and_chorus {
            Some(Chorus::new(settings.sample_rate, 0.002, 0.0019, 0.4))
        } else {
            None
        };
        let chorus_input_left = if settings.enable_reverb_and_chorus {
            Some(vec![0_f32; settings.block_size as usize])
        } else {
            None
        };
        let chorus_input_right = if settings.enable_reverb_and_chorus {
            Some(vec![0_f32; settings.block_size as usize])
        } else {
            None
        };
        let chorus_output_left = if settings.enable_reverb_and_chorus {
            Some(vec![0_f32; settings.block_size as usize])
        } else {
            None
        };
        let chorus_output_right = if settings.enable_reverb_and_chorus {
            Some(vec![0_f32; settings.block_size as usize])
        } else {
            None
        };

        Ok(Self {
            sound_font: Arc::clone(sound_font),
            sample_rate: settings.sample_rate,
            block_size: settings.block_size,
            maximum_polyphony: settings.maximum_polyphony,
            enable_reverb_and_chorus: settings.enable_reverb_and_chorus,
            minimum_voice_duration,
            preset_lookup,
            default_preset,
            channels,
            voices,
            block_left,
            block_right,
            inverse_block_size,
            block_read,
            master_volume,
            reverb,
            reverb_input,
            reverb_output_left,
            reverb_output_right,
            chorus,
            chorus_input_left,
            chorus_input_right,
            chorus_output_left,
            chorus_output_right,
        })
    }

    pub fn process_midi_message(&mut self, channel: i32, command: i32, data1: i32, data2: i32) {
        if !(0 <= channel && channel < self.channels.len() as i32) {
            return;
        }

        let channel_info = &mut self.channels[channel as usize];

        match command {
            0x80 => self.note_off(channel, data1),       // Note Off
            0x90 => self.note_on(channel, data1, data2), // Note On
            0xB0 => match data1 // Controller
            {
                0x00 => channel_info.set_bank(data2), // Bank Selection
                0x01 => channel_info.set_modulation_coarse(data2), // Modulation Coarse
                0x21 => channel_info.set_modulation_fine(data2), // Modulation Fine
                0x06 => channel_info.data_entry_coarse(data2), // Data Entry Coarse
                0x26 => channel_info.data_entry_fine(data2), // Data Entry Fine
                0x07 => channel_info.set_volume_coarse(data2), // Channel Volume Coarse
                0x27 => channel_info.set_volume_fine(data2), // Channel Volume Fine
                0x0A => channel_info.set_pan_coarse(data2), // Pan Coarse
                0x2A => channel_info.set_pan_fine(data2), // Pan Fine
                0x0B => channel_info.set_expression_coarse(data2), // Expression Coarse
                0x2B => channel_info.set_expression_fine(data2), // Expression Fine
                0x40 => channel_info.set_hold_pedal(data2), // Hold Pedal
                0x5B => channel_info.set_reverb_send(data2), // Reverb Send
                0x5D => channel_info.set_chorus_send(data2), // Chorus Send
                0x65 => channel_info.set_rpn_coarse(data2), // RPN Coarse
                0x64 => channel_info.set_rpn_fine(data2), // RPN Fine
                0x78 => self.note_off_all_channel(channel, true), // All Sound Off
                0x79 => self.reset_all_controllers_channel(channel), // Reset All Controllers
                0x7B => self.note_off_all_channel(channel, false), // All Note Off
                _ => (),
            },
            0xC0 => channel_info.set_patch(data1), // Program Change
            0xE0 => channel_info.set_pitch_bend(data1, data2), // Pitch Bend
            _ => (),
        }
    }

    pub fn note_off(&mut self, channel: i32, key: i32) {
        if !(0 <= channel && channel < self.channels.len() as i32) {
            return;
        }

        for i in 0..self.voices.active_voice_count as usize {
            let voice = self.voices.voices[i].as_mut();
            if voice.channel == channel && voice.key == key {
                voice.end();
            }
        }
    }

    pub fn note_on(&mut self, channel: i32, key: i32, velocity: i32) {
        if velocity == 0 {
            self.note_off(channel, key);
            return;
        }

        if !(0 <= channel && channel < self.channels.len() as i32) {
            return;
        }

        let channel_info = &self.channels[channel as usize];

        let preset_id = (channel_info.get_bank_number() << 16) | channel_info.get_patch_number();

        let mut preset = self.default_preset;
        match self.preset_lookup.get(&preset_id) {
            Some(value) => preset = *value,
            None => {
                // Try fallback to the GM sound set.
                // Normally, the given patch number + the bank number 0 will work.
                // For drums (bank number >= 128), it seems to be better to select the standard set (128:0).
                let gm_preset_id = if channel_info.get_bank_number() < 128 {
                    channel_info.get_patch_number()
                } else {
                    128 << 16
                };
                match self.preset_lookup.get(&gm_preset_id) {
                    Some(value) => preset = *value,
                    None => (), // No corresponding preset was found. Use the default one...
                }
            }
        }

        let preset = &self.sound_font.presets[preset];
        for pr in 0..preset.regions.len() {
            let preset_region = &preset.regions[pr];
            if preset_region.contains(key, velocity) {
                let instrument = &self.sound_font.instruments[preset_region.instrument];
                for ir in 0..instrument.regions.len() {
                    let instrument_region = &instrument.regions[ir];
                    if instrument_region.contains(key, velocity) {
                        let region_pair = RegionPair::new(preset_region, instrument_region);

                        match self.voices.request_new(instrument_region, channel) {
                            Some(value) => value.start(&region_pair, channel, key, velocity),
                            None => (),
                        }
                    }
                }
            }
        }
    }

    pub fn note_off_all(&mut self, immediate: bool) {
        if immediate {
            self.voices.clear();
        } else {
            for i in 0..self.voices.active_voice_count as usize {
                self.voices.voices[i].end();
            }
        }
    }

    pub fn note_off_all_channel(&mut self, channel: i32, immediate: bool) {
        if immediate {
            for i in 0..self.voices.active_voice_count as usize {
                let voice = self.voices.voices[i].as_mut();
                if voice.channel == channel {
                    voice.kill();
                }
            }
        } else {
            for i in 0..self.voices.active_voice_count as usize {
                let voice = self.voices.voices[i].as_mut();
                if voice.channel == channel {
                    voice.end();
                }
            }
        }
    }

    pub fn reset_all_controllers(&mut self) {
        for channel in &mut self.channels {
            channel.reset_all_controllers();
        }
    }

    pub fn reset_all_controllers_channel(&mut self, channel: i32) {
        if !(0 <= channel && channel < self.channels.len() as i32) {
            return;
        }

        self.channels[channel as usize].reset_all_controllers();
    }

    pub fn reset(&mut self) {
        self.voices.clear();

        for channel in &mut self.channels {
            channel.reset();
        }

        if self.enable_reverb_and_chorus {
            self.reverb.as_mut().unwrap().mute();
            self.chorus.as_mut().unwrap().mute();
        }

        self.block_read = self.block_size as usize;
    }

    pub fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        if left.len() != right.len() {
            panic!("The output buffers for the left and right must be the same length.");
        }

        let left_length = left.len();

        let mut wrote = 0;
        while wrote < left_length {
            if self.block_read == self.block_size as usize {
                self.render_block();
                self.block_read = 0;
            }

            let src_rem = self.block_size as usize - self.block_read;
            let dst_rem = left_length - wrote;
            let rem = cmp::min(src_rem, dst_rem);

            for t in 0..rem {
                left[wrote + t] = self.block_left[self.block_read + t];
                right[wrote + t] = self.block_right[self.block_read + t];
            }

            self.block_read += rem;
            wrote += rem;
        }
    }

    fn render_block(&mut self) {
        self.voices
            .process(&self.sound_font.wave_data, &self.channels);

        for t in 0..self.block_size as usize {
            self.block_left[t] = 0_f32;
            self.block_right[t] = 0_f32;
        }

        for i in 0..self.voices.active_voice_count as usize {
            let voice = self.voices.voices[i].as_ref();
            let previous_gain_left = self.master_volume * voice.previous_mix_gain_left;
            let current_gain_left = self.master_volume * voice.current_mix_gain_left;
            Synthesizer::write_block(
                previous_gain_left,
                current_gain_left,
                &voice.block[..],
                &mut self.block_left[..],
                self.inverse_block_size,
            );
            let previous_gain_right = self.master_volume * voice.previous_mix_gain_right;
            let current_gain_right = self.master_volume * voice.current_mix_gain_right;
            Synthesizer::write_block(
                previous_gain_right,
                current_gain_right,
                &voice.block[..],
                &mut self.block_right[..],
                self.inverse_block_size,
            );
        }

        if self.enable_reverb_and_chorus {
            let chorus = self.chorus.as_mut().unwrap();
            let chorus_input_left = self.chorus_input_left.as_mut().unwrap();
            let chorus_input_right = self.chorus_input_right.as_mut().unwrap();
            let chorus_output_left = self.chorus_output_left.as_mut().unwrap();
            let chorus_output_right = self.chorus_output_right.as_mut().unwrap();
            for i in 0..self.block_size as usize {
                chorus_input_left[i] = 0_f32;
                chorus_input_right[i] = 0_f32;
            }
            for i in 0..self.voices.active_voice_count as usize {
                let voice = self.voices.voices[i].as_ref();
                let previous_gain_left = voice.previous_chorus_send * voice.previous_mix_gain_left;
                let current_gain_left = voice.current_chorus_send * voice.current_mix_gain_left;
                Synthesizer::write_block(
                    previous_gain_left,
                    current_gain_left,
                    &voice.block[..],
                    &mut chorus_input_left[..],
                    self.inverse_block_size,
                );
                let previous_gain_right =
                    voice.previous_chorus_send * voice.previous_mix_gain_right;
                let current_gain_right = voice.current_chorus_send * voice.current_mix_gain_right;
                Synthesizer::write_block(
                    previous_gain_right,
                    current_gain_right,
                    &voice.block[..],
                    &mut chorus_input_right[..],
                    self.inverse_block_size,
                );
            }
            chorus.process(
                chorus_input_left,
                chorus_input_right,
                chorus_output_left,
                chorus_output_right,
            );
            ArrayMath::multiply_add(
                self.master_volume,
                chorus_output_left,
                &mut self.block_left[..],
            );
            ArrayMath::multiply_add(
                self.master_volume,
                chorus_output_right,
                &mut self.block_right[..],
            );

            let reverb = self.reverb.as_mut().unwrap();
            let reverb_input = self.reverb_input.as_mut().unwrap();
            let reverb_output_left = self.reverb_output_left.as_mut().unwrap();
            let reverb_output_right = self.reverb_output_right.as_mut().unwrap();
            for i in 0..self.block_size as usize {
                reverb_input[i] = 0_f32;
            }
            for i in 0..self.voices.active_voice_count as usize {
                let voice = self.voices.voices[i].as_ref();
                let previous_gain = reverb.get_input_gain()
                    * voice.previous_reverb_send
                    * (voice.previous_mix_gain_left + voice.previous_mix_gain_right);
                let current_gain = reverb.get_input_gain()
                    * voice.current_reverb_send
                    * (voice.current_mix_gain_left + voice.current_mix_gain_right);
                Synthesizer::write_block(
                    previous_gain,
                    current_gain,
                    &voice.block[..],
                    &mut reverb_input[..],
                    self.inverse_block_size,
                );
            }

            reverb.process(reverb_input, reverb_output_left, reverb_output_right);
            ArrayMath::multiply_add(
                self.master_volume,
                reverb_output_left,
                &mut self.block_left[..],
            );
            ArrayMath::multiply_add(
                self.master_volume,
                reverb_output_right,
                &mut self.block_right[..],
            );
        }
    }

    fn write_block(
        previous_gain: f32,
        current_gain: f32,
        source: &[f32],
        destination: &mut [f32],
        inverse_block_size: f32,
    ) {
        if SoundFontMath::max(previous_gain, current_gain) < SoundFontMath::NON_AUDIBLE {
            return;
        }

        if (current_gain - previous_gain).abs() < 1.0E-3_f32 {
            ArrayMath::multiply_add(current_gain, source, destination);
        } else {
            let step = inverse_block_size * (current_gain - previous_gain);
            ArrayMath::multiply_add_slope(previous_gain, step, source, destination);
        }
    }

    pub fn get_sound_font(&self) -> &SoundFont {
        &self.sound_font
    }

    pub fn get_sample_rate(&self) -> i32 {
        self.sample_rate
    }

    pub fn get_maximum_polyphony(&self) -> i32 {
        self.maximum_polyphony
    }

    pub fn get_enable_reverb_and_chorus(&self) -> bool {
        self.enable_reverb_and_chorus
    }

    pub fn get_master_volume(&self) -> f32 {
        self.master_volume
    }

    pub fn set_master_volume(&mut self, master_volume: f32) {
        self.master_volume = master_volume;
    }
}
