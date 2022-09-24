use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;
use rustysynth::SoundFont;
use rustysynth::SynthesizerSettings;
use rustysynth::Synthesizer;
use rustysynth::MidiFile;

fn main()
{
    println!("START");

    let sound_font_path = Path::new("TimGM6mb.sf2");
    let mut sound_font_file = File::open(&sound_font_path).unwrap();

    let sound_font = Rc::new(SoundFont::new(&mut sound_font_file).unwrap());
    let settings = SynthesizerSettings::new(44100);
    let mut synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();

    synthesizer.note_on(0, 60, 100);
    synthesizer.note_on(0, 64, 100);
    synthesizer.note_on(0, 67, 100);

    let mut left: Vec<f32> = vec![0_f32; 3 * settings.sample_rate as usize];
    let mut right: Vec<f32> = vec![0_f32; 3 * settings.sample_rate as usize];
    synthesizer.render(&mut left[..], &mut right[..]);

    let mut buffer: Vec<u8> = vec![0; 4 * left.len()];
    for t in 0..left.len()
    {
        let left_i16 = (left[t] * 32768_f32) as i16;
        let right_i16 = (right[t] * 32768_f32) as i16;
        let offset = 4 * t;
        buffer[offset + 0] = left_i16 as u8;
        buffer[offset + 1] = (left_i16 >> 8) as u8;
        buffer[offset + 2] = right_i16 as u8;
        buffer[offset + 3] = (right_i16 >> 8) as u8;
    }

    let pcm_path = Path::new("test.pcm");
    let mut pcm_file = File::create(&pcm_path).unwrap();
    pcm_file.write_all(&buffer[..]);

    println!("END");
}
