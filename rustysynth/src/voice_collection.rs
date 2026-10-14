#![allow(dead_code)]

use crate::bi_quad_filter::BiQuadFilter;
use crate::channel::Channel;
use crate::instrument_region::InstrumentRegion;
use crate::synthesizer_settings::SynthesizerSettings;
use crate::voice::Voice;

#[derive(Debug)]
#[non_exhaustive]
pub(crate) struct VoiceCollection {
    // The voices allocated so far: `initial_polyphony` of them at the start, then more on demand, up to `maximum_polyphony`.
    voices: Vec<Voice>,
    maximum_polyphony: usize,
    // The only settings a voice reads, kept to make more voices later.
    sample_rate: i32,
    block_size: usize,
    pub(crate) active_voice_count: usize,
}

impl VoiceCollection {
    pub(crate) fn new(settings: &SynthesizerSettings) -> Self {
        let initial = settings
            .initial_polyphony
            .unwrap_or(settings.maximum_polyphony)
            .min(settings.maximum_polyphony);
        Self {
            voices: (0..initial).map(|_| Voice::new(settings)).collect(),
            maximum_polyphony: settings.maximum_polyphony,
            sample_rate: settings.sample_rate,
            block_size: settings.block_size,
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
            for i in 0..self.active_voice_count {
                let voice = &self.voices[i];
                if voice.exclusive_class() == exclusive_class && voice.channel() == channel {
                    return Some(&mut self.voices[i]);
                }
            }
        }

        // If the number of active voices is less than the limit, use a free one (allocating one if all existing voices are in use).
        if self.active_voice_count == self.voices.len()
            && self.voices.len() < self.maximum_polyphony
        {
            let mut settings = SynthesizerSettings::new(self.sample_rate);
            settings.block_size = self.block_size;
            self.voices.push(Voice::new(&settings));
        }
        if (self.active_voice_count) < self.voices.len() {
            let i = self.active_voice_count;
            self.active_voice_count += 1;
            return Some(&mut self.voices[i]);
        }

        // Too many active voices...
        // Find one which has the lowest priority.
        let mut candidate: usize = 0;
        let mut lowest_priority = f32::MAX;
        for i in 0..self.active_voice_count {
            let voice = &self.voices[i];
            let priority = voice.priority();
            if priority < lowest_priority {
                lowest_priority = priority;
                candidate = i;
            } else if priority == lowest_priority {
                // Same priority...
                // The older one should be more suitable for reuse.
                if voice.voice_length() > self.voices[candidate].voice_length() {
                    candidate = i;
                }
            }
        }
        Some(&mut self.voices[candidate])
    }

    /// Runs every voice up to the filter (finished voices are removed), then all the filters four voices at a time, then the rest of each voice.
    pub(crate) fn process(&mut self, data: &[i16], channels: &[Channel]) {
        let mut i: usize = 0;
        while i < self.active_voice_count {
            if self.voices[i].process_before_filter(data, channels) {
                i += 1;
            } else {
                self.active_voice_count -= 1;
                self.voices.swap(i, self.active_voice_count);
            }
        }

        BiQuadFilter::process_many(
            self.voices[0..self.active_voice_count]
                .iter_mut()
                .map(|v| v.filter_and_block()),
        );

        for voice in self.voices[0..self.active_voice_count].iter_mut() {
            voice.process_after_filter(channels);
        }
    }

    pub(crate) fn get_active_voices(&mut self) -> &mut [Voice] {
        &mut self.voices[0..self.active_voice_count]
    }

    pub(crate) fn clear(&mut self) {
        self.active_voice_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_polyphony() {
        let mut settings = SynthesizerSettings::new(44100);
        settings.maximum_polyphony = 100;
        assert_eq!(
            VoiceCollection::new(&settings).voices.len(),
            100,
            "all of them by default"
        );
        settings.initial_polyphony = Some(10);
        assert_eq!(VoiceCollection::new(&settings).voices.len(), 10);
        settings.initial_polyphony = Some(0);
        assert_eq!(VoiceCollection::new(&settings).voices.len(), 0);
        settings.initial_polyphony = Some(1000);
        assert_eq!(
            VoiceCollection::new(&settings).voices.len(),
            100,
            "never more than the limit"
        );
        settings.maximum_polyphony = 1_000_000;
        settings.initial_polyphony = Some(64);
        let voices = VoiceCollection::new(&settings);
        assert_eq!((voices.voices.len(), voices.active_voice_count), (64, 0));
    }
}
