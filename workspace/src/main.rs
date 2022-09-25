use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use sfml::audio::SoundStream;
use sfml::audio::SoundStreamPlayer;
use sfml::system;
use sfml::system::Time;
use rustysynth::SoundFont;
use rustysynth::SynthesizerSettings;
use rustysynth::Synthesizer;
use rustysynth::MidiFile;
use rustysynth::MidiFileSequencer;

struct MidiMusicStream
{
    sequencer: MidiFileSequencer,
    left: Vec<f32>,
    right: Vec<f32>,
    batch: Vec<i16>,
}

impl MidiMusicStream
{
    const SAMPLE_RATE: u32 = 44100;
    const SAMPLE_MIN: i32 = i16::MIN as i32;
    const SAMPLE_MAX: i32 = i16::MAX as i32;

    fn new(sequencer: MidiFileSequencer) -> Self
    {
        let batch_length = (MidiMusicStream::SAMPLE_RATE / 20) as usize;

        Self
        {
            sequencer: sequencer,
            left: vec![0_f32; batch_length],
            right: vec![0_f32; batch_length],
            batch: vec![0; 2 * batch_length],
        }
    }
}

impl SoundStream for MidiMusicStream
{
    fn get_data(&mut self) -> (&mut [i16], bool)
    {
        self.sequencer.render(&mut self.left[..], &mut self.right[..]);

        let length = self.left.len();
        for t in 0..length
        {
            let mut sample_left = (32768_f32 * self.left[t]) as i32;
            if sample_left < MidiMusicStream::SAMPLE_MIN { sample_left = MidiMusicStream::SAMPLE_MIN };
            if sample_left > MidiMusicStream::SAMPLE_MAX { sample_left = MidiMusicStream::SAMPLE_MAX };
            let sample_left = sample_left as i16;

            let mut sample_right = (32768_f32 * self.right[t]) as i32;
            if sample_right < MidiMusicStream::SAMPLE_MIN { sample_right = MidiMusicStream::SAMPLE_MIN };
            if sample_right > MidiMusicStream::SAMPLE_MAX { sample_right = MidiMusicStream::SAMPLE_MAX };
            let sample_right = sample_right as i16;

            let offset = 2 * t;
            self.batch[offset + 0] = sample_left;
            self.batch[offset + 1] = sample_right;
        }

        (&mut self.batch[..], true)
    }

    fn seek(&mut self, _offset: Time)
    {
    }

    fn channel_count(&self) -> u32
    {
        2
    }

    fn sample_rate(&self) -> u32
    {
        MidiMusicStream::SAMPLE_RATE
    }
}

fn main()
{
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

    // Start the sound stream.
    let mut stream = MidiMusicStream::new(sequencer);
    let mut player = SoundStreamPlayer::new(&mut stream);
    player.play();

    print!("PLAYING");

    let mut count: i32 = 0;
    while count < 300
    {
        // Leave some CPU time for other processes.
        system::sleep(Time::milliseconds(100));
        
        print!(".");
        let _ = std::io::stdout().flush();

        count += 1;
    }
    println!();
}
