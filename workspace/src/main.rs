use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use std::sync::Mutex;
use sfml::audio::SoundStream;
use sfml::audio::SoundStreamPlayer;
use sfml::graphics::Color;
use sfml::graphics::PrimitiveType;
use sfml::graphics::RenderStates;
use sfml::graphics::RenderTarget;
use sfml::graphics::RenderWindow;
use sfml::graphics::Vertex;
use sfml::system;
use sfml::system::Time;
use sfml::system::Vector2;
use sfml::window;
use sfml::window::Event;
use sfml::window::Style;
use rustysynth::SoundFont;
use rustysynth::SynthesizerSettings;
use rustysynth::Synthesizer;
use rustysynth::MidiFile;
use rustysynth::MidiFileSequencer;

const WAVEFORM_LENGTH: usize = 1024;

struct MidiMusicStream
{
    sequencer: MidiFileSequencer,
    left: Vec<f32>,
    right: Vec<f32>,
    batch: Vec<i16>,
    mutex: Rc<Mutex<Vec<f32>>>,
}

impl MidiMusicStream
{
    const SAMPLE_RATE: u32 = 44100;
    const SAMPLE_MIN: i32 = i16::MIN as i32;
    const SAMPLE_MAX: i32 = i16::MAX as i32;

    fn new(sequencer: MidiFileSequencer, mutex: Rc<Mutex<Vec<f32>>>) -> Self
    {
        let batch_length = (MidiMusicStream::SAMPLE_RATE / 20) as usize;

        Self
        {
            sequencer: sequencer,
            left: vec![0_f32; batch_length],
            right: vec![0_f32; batch_length],
            batch: vec![0; 2 * batch_length],
            mutex: mutex,
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

        let batch_length = (MidiMusicStream::SAMPLE_RATE / 20) as usize;

        let mut a = self.mutex.lock().unwrap();
        for i in 0..WAVEFORM_LENGTH
        {
            let p: f64 = (i as f64) / (WAVEFORM_LENGTH as f64) * (batch_length as f64);
            let j = p as usize;
            a[i] = self.left[j] + self.right[j];
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
    let mut window = RenderWindow::new(
        (1024, 768),
        "MIDI Music Playback",
        Style::TITLEBAR | Style::CLOSE,
        &Default::default(),
    );

    window.set_framerate_limit(60);

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

    let wav = vec![0_f32; WAVEFORM_LENGTH];
    let mutex = Rc::new(Mutex::new(wav));
    let mutex2 = mutex.clone();

    // Start the sound stream.
    let mut stream = MidiMusicStream::new(sequencer, mutex);
    let mut player = SoundStreamPlayer::new(&mut stream);
    player.play();

    let mut waveform = vec![0_f32; WAVEFORM_LENGTH];

    while window.is_open()
    {
        while let Some(event) = window.poll_event()
        {
            match event
            {
                Event::Closed => window.close(),
                _ => {}
            }
        }

        window.clear(Color::rgb(0, 32, 64));

        {
            let a = mutex2.lock().unwrap();
            for i in 0..WAVEFORM_LENGTH
            {
                waveform[i] = 0.5_f32 * waveform[i] + 0.5_f32 * a[i];
            }
        }
        draw_waveform(&mut window, &waveform);

        window.display();
    }
}

fn draw_waveform(window: &mut RenderWindow, data: &[f32])
{
    let mut vs: [Vertex; 4 * WAVEFORM_LENGTH] = [Vertex::default(); 4 * WAVEFORM_LENGTH];

    for i in 0..WAVEFORM_LENGTH
    {
        let offset = 4 * i;
        let val = data[i].abs();
        let col = Color::rgb(0, 100, 200);
        vs[offset + 0].color = col;
        vs[offset + 0].position = Vector2::new((i + 0) as f32, -300_f32 * val + 384_f32);
        vs[offset + 1].color = col;
        vs[offset + 1].position = Vector2::new((i + 1) as f32, -300_f32 * val + 384_f32);
        vs[offset + 2].color = col;
        vs[offset + 2].position = Vector2::new((i + 1) as f32, 300_f32 * val + 384_f32);
        vs[offset + 3].color = col;
        vs[offset + 3].position = Vector2::new((i + 0) as f32, 300_f32 * val + 384_f32);
    }

    window.draw_primitives(&vs[..], PrimitiveType::QUADS, &RenderStates::DEFAULT);
}
