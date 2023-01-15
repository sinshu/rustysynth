use rustysynth::MidiFile;
use rustysynth::MidiFileSequencer;
use rustysynth::SoundFont;
use rustysynth::Synthesizer;
use rustysynth::SynthesizerSettings;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

fn main() {
    simple_chord();
    flourish();
}

fn simple_chord() {
    // Load the SoundFont.
    let mut sf2 = File::open("TimGM6mb.sf2").unwrap();
    let sound_font = Rc::new(SoundFont::new(&mut sf2).unwrap());

    // Create the synthesizer.
    let settings = SynthesizerSettings::new(44100);
    let mut synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();

    // Play some notes (middle C, E, G).
    synthesizer.note_on(0, 60, 100);
    synthesizer.note_on(0, 64, 100);
    synthesizer.note_on(0, 67, 100);

    // The output buffer (3 seconds).
    let sample_count = (3 * settings.sample_rate) as usize;
    let mut left: Vec<f32> = vec![0_f32; sample_count];
    let mut right: Vec<f32> = vec![0_f32; sample_count];

    // Render the waveform.
    synthesizer.render(&mut left[..], &mut right[..]);

    // Write the waveform to the file.
    write_pcm(&left[..], &right[..], "simple_chord.pcm");
}

fn flourish() {
    // Load the SoundFont.
    let mut sf2 = File::open("TimGM6mb.sf2").unwrap();
    let sound_font = Rc::new(SoundFont::new(&mut sf2).unwrap());

    // Load the MIDI file.
    let mut mid = File::open("flourish.mid").unwrap();
    let midi_file = Rc::new(MidiFile::new(&mut mid).unwrap());

    // Create the MIDI file sequencer.
    let settings = SynthesizerSettings::new(44100);
    let synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();
    let mut sequencer = MidiFileSequencer::new(synthesizer);

    // Play the MIDI file.
    sequencer.play(&midi_file, false);

    // The output buffer.
    let sample_count = (settings.sample_rate as f64 * midi_file.get_length()) as usize;
    let mut left: Vec<f32> = vec![0_f32; sample_count];
    let mut right: Vec<f32> = vec![0_f32; sample_count];

    // Render the waveform.
    sequencer.render(&mut left[..], &mut right[..]);

    // Write the waveform to the file.
    write_pcm(&left[..], &right[..], "flourish.pcm");
}

fn write_pcm(left: &[f32], right: &[f32], path: &str) {
    let mut max: f32 = 0_f32;
    for t in 0..left.len() {
        if left[t].abs() > max {
            max = left[t].abs();
        }
        if right[t].abs() > max {
            max = right[t].abs();
        }
    }
    let a = 0.99_f32 / max;

    let mut buf: Vec<u8> = vec![0; 4 * left.len()];
    for t in 0..left.len() {
        let left_i16 = (a * left[t] * 32768_f32) as i16;
        let right_i16 = (a * right[t] * 32768_f32) as i16;

        let offset = 4 * t;
        buf[offset + 0] = left_i16 as u8;
        buf[offset + 1] = (left_i16 >> 8) as u8;
        buf[offset + 2] = right_i16 as u8;
        buf[offset + 3] = (right_i16 >> 8) as u8;
    }

    let mut pcm = File::create(path).unwrap();
    pcm.write_all(&buf[..]).unwrap();
}
