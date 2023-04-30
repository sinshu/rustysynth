use rustysynth::MidiFile;
use rustysynth::MidiFileSequencer;
use rustysynth::SoundFont;
use rustysynth::Synthesizer;
use rustysynth::SynthesizerSettings;
use std::fs::File;
use std::sync::Arc;
use tinyaudio::prelude::*;

fn main() {
    // Setup the audio output.
    let params = OutputDeviceParameters {
        channels_count: 2,
        sample_rate: 44100,
        channel_sample_count: 4410,
    };

    // Buffer for the audio output.
    let mut left: Vec<f32> = vec![0_f32; params.channel_sample_count];
    let mut right: Vec<f32> = vec![0_f32; params.channel_sample_count];

    // Load the SoundFont.
    let mut sf2 = File::open("TimGM6mb.sf2").unwrap();
    let sound_font = Arc::new(SoundFont::new(&mut sf2).unwrap());

    // Load the MIDI file.
    let mut mid = File::open("flourish.mid").unwrap();
    let midi_file = Arc::new(MidiFile::new(&mut mid).unwrap());

    // Create the MIDI file sequencer.
    let settings = SynthesizerSettings::new(params.sample_rate as i32);
    let synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();
    let mut sequencer = MidiFileSequencer::new(synthesizer);

    // Play the MIDI file.
    sequencer.play(&midi_file, false);

    // Start the audio output.
    let _device = run_output_device(params, {
        move |data| {
            sequencer.render(&mut left[..], &mut right[..]);
            for i in 0..params.channel_sample_count {
                let offset = 2 * i;
                data[offset] = left[i];
                data[offset + 1] = right[i];
            }
        }
    })
    .unwrap();

    // Wait for 10 seconds.
    std::thread::sleep(std::time::Duration::from_secs(10));
}
