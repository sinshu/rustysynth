use crate::instrument_region::InstrumentRegion;
use crate::synthesizer::Synthesizer;
use crate::synthesizer_settings::SynthesizerSettings;
use crate::voice::Voice;

#[non_exhaustive]
struct VoiceCollection
{
    voices: Vec<Box<Voice>>,
    pub(crate) active_voice_count: i32,
}

impl VoiceCollection
{
    pub(crate) fn new(settings: &SynthesizerSettings) -> Self
    {
        let mut voices: Vec<Box<Voice>> = Vec::new();
        for _i in 0..settings.maximum_polyphony
        {
            voices.push(Box::new(Voice::new(settings)));
        }

        Self
        {
            voices: voices,
            active_voice_count: 0,
        }
    }

    pub(crate) fn request_new(&mut self, region: &InstrumentRegion, channel: i32) -> Option<&Voice>
    {
        // If an exclusive class is assigned to the region, find a voice with the same class.
        // If found, reuse it to avoid playing multiple voices with the same class at a time.
        let exclusive_class = region.get_exclusive_class();
        if exclusive_class != 0
        {
            for i in 0..self.active_voice_count as usize
            {
                let voice = self.voices[i].as_ref();
                if voice.exclusive_class == exclusive_class && voice.channel == channel
                {
                    return Some(voice);
                }
            }
        }

        // If the number of active voices is less than the limit, use a free one.
        if (self.active_voice_count as usize) < self.voices.len()
        {
            let free = self.voices[self.active_voice_count as usize].as_ref();
            self.active_voice_count += 1;
            return Some(free);
        }

        // Too many active voices...
        // Find one which has the lowest priority.
        let mut candidate: Option<&Voice> = None;
        let mut lowest_priority = f32::MAX;
        for i in 0..self.active_voice_count as usize
        {
            let voice = self.voices[i].as_ref();
            let priority = voice.get_priority();
            if priority < lowest_priority
            {
                lowest_priority = priority;
                candidate = Some(voice);
            }
            else if (priority == lowest_priority)
            {
                // Same priority...
                // The older one should be more suitable for reuse.
                if voice.voice_length > candidate.unwrap().voice_length
                {
                    candidate = Some(voice);
                }
            }
        }
        return candidate;
    }

    pub(crate) fn process(&mut self, synthesizer: &Synthesizer)
    {
        let mut i: usize = 0;

        loop
        {
            if i == self.active_voice_count as usize
            {
                return;
            }

            if self.voices[i].as_mut().process(synthesizer)
            {
                i += 1;
            }
            else
            {
                self.active_voice_count -= 1;
                self.voices.swap(i, self.active_voice_count as usize);
            }
        }
    }

    pub(crate) fn clear(&mut self)
    {
        self.active_voice_count = 0;
    }

    pub(crate) fn get_active_voices(&self) -> &[Box<Voice>]
    {
        &self.voices[0..self.active_voice_count as usize]
    }
}
