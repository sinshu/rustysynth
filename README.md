# RustySynth

RustySynth is a SoundFont MIDI synthesizer written in pure Rust, ported from [MeltySynth](https://github.com/sinshu/meltysynth).



## Demo

https://www.youtube.com/watch?v=JzXjQqKzkWQ

[![Youtube video](https://img.youtube.com/vi/JzXjQqKzkWQ/0.jpg)](https://www.youtube.com/watch?v=JzXjQqKzkWQ)



## Examples

An example code to synthesize a simple chord:

```rust
// Load the SoundFont.
let sf2_path = Path::new("TimGM6mb.sf2");
let mut sf2_reader = File::open(&sf2_path).unwrap();
let sound_font = Rc::new(SoundFont::new(&mut sf2_reader).unwrap());

// Create the synthesizer.
let sample_rate = 44100;
let settings = SynthesizerSettings::new(sample_rate);
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
```

Another example code to synthesize a MIDI file:

```rust
// Load the SoundFont.
let sf2_path = Path::new("TimGM6mb.sf2");
let mut sf2_reader = File::open(&sf2_path).unwrap();
let sound_font = Rc::new(SoundFont::new(&mut sf2_reader).unwrap());

// Load the MIDI file.
let mid_path = Path::new("flourish.mid");
let mut mid_reader = File::open(&mid_path).unwrap();
let midi_file = Rc::new(MidiFile::new(&mut mid_reader).unwrap());

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
```



## Todo

* __Wave synthesis__
    - [x] SoundFont reader
    - [x] Waveform generator
    - [x] Envelope generator
    - [x] Low-pass filter
    - [x] Vibrato LFO
    - [x] Modulation LFO
* __MIDI message processing__
    - [x] Note on/off
    - [x] Bank selection
    - [x] Modulation
    - [x] Volume control
    - [x] Pan
    - [x] Expression
    - [x] Hold pedal
    - [x] Program change
    - [x] Pitch bend
    - [x] Tuning
* __Effects__
    - [ ] Reverb
    - [ ] Chorus
* __Other things__
    - [x] Standard MIDI file support
    - [ ] Loop extension support
    - [x] Performace optimization



## License

RustySynth is available under [the MIT license](LICENSE.txt).
