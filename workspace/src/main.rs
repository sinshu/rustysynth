use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;
use rustysynth::SoundFont;
use rustysynth::SynthesizerSettings;
use rustysynth::Synthesizer;
use rustysynth::MidiFile;
use rustysynth::MidiFileSequencer;

fn main()
{
    let sf2_path = Path::new("RLNDGM.SF2");
    let mut sf2_reader = File::open(&sf2_path).unwrap();
    let sound_font = Rc::new(SoundFont::new(&mut sf2_reader).unwrap());

    let mid_path = Path::new("town.mid");
    let mut mid_reader = File::open(&mid_path).unwrap();
    let midi_file = Rc::new(MidiFile::new(&mut mid_reader).unwrap());

    let settings = SynthesizerSettings::new(44100);
    let mut synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();
    let mut sequencer = MidiFileSequencer::new(&synthesizer);

    sequencer.play(&mut synthesizer, &midi_file, false);

    let sample_count = (settings.sample_rate as f64 * midi_file.get_length()) as usize;
    let mut left: Vec<f32> = vec![0_f32; sample_count];
    let mut right: Vec<f32> = vec![0_f32; sample_count];
    sequencer.render(&mut synthesizer, &mut left[..], &mut right[..]);

    let mut max: f32 = 0_f32;
    for t in 0..left.len()
    {
        if left[t].abs() > max { max = left[t].abs(); }
        if right[t].abs() > max { max = right[t].abs(); }
    }
    let a = 0.99_f32 / max;

    let mut buffer: Vec<u8> = vec![0; 4 * left.len()];
    for t in 0..left.len()
    {
        let left_i16 = (a * left[t] * 32768_f32) as i16;
        let right_i16 = (a * right[t] * 32768_f32) as i16;
        let offset = 4 * t;
        buffer[offset + 0] = left_i16 as u8;
        buffer[offset + 1] = (left_i16 >> 8) as u8;
        buffer[offset + 2] = right_i16 as u8;
        buffer[offset + 3] = (right_i16 >> 8) as u8;
    }

    let pcm_path = Path::new("out.pcm");
    let mut pcm_writer = File::create(&pcm_path).unwrap();
    pcm_writer.write_all(&buffer[..]).unwrap();
}
