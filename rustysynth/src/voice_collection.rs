#![allow(dead_code)]

use crate::channel::Channel;
use crate::instrument_region::InstrumentRegion;
use crate::synthesizer_settings::SynthesizerSettings;
use crate::voice::Voice;

#[non_exhaustive]
pub(crate) struct VoiceCollection {
    pub(crate) voices: Vec<Box<Voice>>,
    pub(crate) active_voice_count: i32,
}

impl VoiceCollection {
    pub(crate) fn new(settings: &SynthesizerSettings) -> Self {
        let mut voices: Vec<Box<Voice>> = Vec::new();
        for _i in 0..settings.maximum_polyphony {
            voices.push(Box::new(Voice::new(settings)));
        }

        Self {
            voices: voices,
            active_voice_count: 0,
        }
    }

    pub(crate) fn request_new(
        &mut self,
        region: &InstrumentRegion,
        channel: i32,
    ) -> Option<&mut Voice> {
        // If an exclusive class is assigned to the region, find a voice with the same class.
        // If found, reuse it to avoid playing multiple voices with the same class at a time.
        let exclusive_class = region.get_exclusive_class();
        if exclusive_class != 0 {
            for i in 0..self.active_voice_count as usize {
                let voice = self.voices[i].as_ref();
                if voice.exclusive_class == exclusive_class && voice.channel == channel {
                    return Some(self.voices[i].as_mut());
                }
            }
        }

        // If the number of active voices is less than the limit, use a free one.
        if (self.active_voice_count as usize) < self.voices.len() {
            let i = self.active_voice_count as usize;
            self.active_voice_count += 1;
            return Some(self.voices[i].as_mut());
        }

        // Too many active voices...
        // Find one which has the lowest priority.
        let mut candidate: usize = 0;
        let mut lowest_priority = f32::MAX;
        for i in 0..self.active_voice_count as usize {
            let voice = self.voices[i].as_ref();
            let priority = voice.get_priority();
            if priority < lowest_priority {
                lowest_priority = priority;
                candidate = i;
            } else if priority == lowest_priority {
                // Same priority...
                // The older one should be more suitable for reuse.
                if voice.voice_length > self.voices[candidate].voice_length {
                    candidate = i;
                }
            }
        }
        return Some(self.voices[candidate].as_mut());
    }

    pub(crate) fn process(&mut self, channels: &Vec<Channel>) {
        let mut i: usize = 0;

        loop {
            if i == self.active_voice_count as usize {
                return;
            }

            if self.voices[i].as_mut().process(channels) {
                i += 1;
            } else {
                self.active_voice_count -= 1;
                self.voices.swap(i, self.active_voice_count as usize);
            }
        }
    }

    pub(crate) fn clear(&mut self) {
        self.active_voice_count = 0;
    }
}
