//! SoundFonts built in memory for the tests of `disarm_unplayable_regions`.
//!
//! Small enough to read in full, and every fault in them is there because a
//! test put it there.

use std::sync::Arc;

use crate::generator_type::GeneratorType;
use crate::{SoundFont, Synthesizer, SynthesizerSettings};

pub(crate) struct Sample {
    pub(crate) name: &'static str,
    pub(crate) pcm: Vec<i16>,
    /// Loop points, relative to the start of this sample.
    pub(crate) loop_start: u32,
    pub(crate) loop_end: u32,
    pub(crate) rate: u32,
    /// Absolute start, end, loop start and loop end to write instead of the
    /// computed ones, for headers that are wrong on purpose.
    pub(crate) raw_bounds: Option<[u32; 4]>,
}

/// A zone: its generators, in order.
pub(crate) type Zone = Vec<(u16, i16)>;

pub(crate) struct Font {
    /// The payload of the `ifil` chunk; four bytes when well formed.
    pub(crate) version: Vec<u8>,
    pub(crate) name: &'static str,
    pub(crate) samples: Vec<Sample>,
    /// Size of an `sm24` chunk to write after the samples, if any.
    pub(crate) sm24: Option<usize>,
    pub(crate) instruments: Vec<(&'static str, Vec<Zone>)>,
    /// Name, bank, patch and zones.
    pub(crate) presets: Vec<(&'static str, u16, u16, Vec<Zone>)>,
}

/// A sawtooth, so that a render can tell sound from silence.
pub(crate) fn tone(frames: usize) -> Vec<i16> {
    (0..frames).map(|i| ((i % 64) as i16 - 32) * 800).collect()
}

impl Font {
    /// One sample of 4000 frames that loops over 1000..3000, one instrument
    /// whose single zone carries `gens` and then the sample, and one preset at
    /// bank 0, program 0.
    pub(crate) fn one_zone(gens: &[(u16, i16)]) -> Self {
        let mut zone = gens.to_vec();
        zone.push((GeneratorType::SAMPLE_ID, 0));
        Font {
            version: vec![2, 0, 1, 0],
            name: "test font",
            samples: vec![Sample {
                name: "tone",
                pcm: tone(4000),
                loop_start: 1000,
                loop_end: 3000,
                rate: 44100,
                raw_bounds: None,
            }],
            sm24: None,
            instruments: vec![("instrument", vec![zone])],
            presets: vec![("preset", 0, 0, vec![vec![(GeneratorType::INSTRUMENT, 0)]])],
        }
    }

    /// Two samples at different rates; an instrument with a global zone and a
    /// key split, a second instrument with an exclusive class; a melodic
    /// preset and a drum kit on bank 128.
    pub(crate) fn several() -> Self {
        let sample = |name, frames: usize, rate| Sample {
            name,
            pcm: tone(frames),
            loop_start: 100,
            loop_end: frames as u32 - 100,
            rate,
            raw_bounds: None,
        };
        let key_range = |lo: u8, hi: u8| (GeneratorType::KEY_RANGE, i16::from_le_bytes([lo, hi]));
        Font {
            version: vec![2, 0, 1, 0],
            name: "several regions",
            samples: vec![sample("low", 3000, 22050), sample("high", 1200, 48000)],
            sm24: None,
            instruments: vec![
                (
                    "split",
                    vec![
                        vec![
                            (GeneratorType::SAMPLE_MODES, 1),
                            (GeneratorType::INITIAL_ATTENUATION, 60),
                        ],
                        vec![key_range(0, 59), (GeneratorType::SAMPLE_ID, 0)],
                        vec![key_range(60, 127), (GeneratorType::SAMPLE_ID, 1)],
                    ],
                ),
                (
                    "kit",
                    vec![vec![
                        (GeneratorType::EXCLUSIVE_CLASS, 1),
                        (GeneratorType::SAMPLE_ID, 1),
                    ]],
                ),
            ],
            presets: vec![
                ("melodic", 0, 0, vec![vec![(GeneratorType::INSTRUMENT, 0)]]),
                ("drums", 128, 0, vec![vec![(GeneratorType::INSTRUMENT, 1)]]),
            ],
        }
    }

    pub(crate) fn build(&self) -> Vec<u8> {
        let mut info = chunk(b"ifil", &self.version);
        info.extend(chunk(b"INAM", &[self.name.as_bytes(), &[0]].concat()));

        let mut smpl = Vec::new();
        let mut shdr = Vec::new();
        for sample in &self.samples {
            let start = (smpl.len() / 2) as u32;
            for value in &sample.pcm {
                smpl.extend_from_slice(&value.to_le_bytes());
            }
            // The 46 zero frames the specification asks for after every sample.
            smpl.extend_from_slice(&[0; 92]);
            let end = start + sample.pcm.len() as u32;
            let bounds = sample.raw_bounds.unwrap_or([
                start,
                end,
                start + sample.loop_start,
                start + sample.loop_end,
            ]);
            shdr.extend(sample_header(sample.name, bounds, sample.rate, 1));
        }
        shdr.extend(sample_header("EOS", [0; 4], 0, 0));

        let mut sdta = chunk(b"smpl", &smpl);
        if let Some(size) = self.sm24 {
            sdta.extend(chunk(b"sm24", &vec![0; size]));
        }

        let instrument_zones: Vec<&Vec<Zone>> = self.instruments.iter().map(|(_, z)| z).collect();
        let (inst_starts, ibag, igen) = bags(&instrument_zones);
        let mut inst = Vec::new();
        for ((name, _), start) in self.instruments.iter().zip(&inst_starts) {
            inst.extend(name20(name));
            inst.extend_from_slice(&start.to_le_bytes());
        }
        inst.extend(name20("EOI"));
        inst.extend_from_slice(&inst_starts.last().unwrap().to_le_bytes());

        let preset_zones: Vec<&Vec<Zone>> = self.presets.iter().map(|(.., z)| z).collect();
        let (preset_starts, pbag, pgen) = bags(&preset_zones);
        let mut phdr = Vec::new();
        for ((name, bank, patch, _), start) in self.presets.iter().zip(&preset_starts) {
            phdr.extend(name20(name));
            for field in [*patch, *bank, *start] {
                phdr.extend_from_slice(&field.to_le_bytes());
            }
            phdr.extend_from_slice(&[0; 12]);
        }
        phdr.extend(name20("EOP"));
        phdr.extend_from_slice(&[0; 4]);
        phdr.extend_from_slice(&preset_starts.last().unwrap().to_le_bytes());
        phdr.extend_from_slice(&[0; 12]);

        let terminal_modulator = [0_u8; 10];
        let mut pdta = Vec::new();
        for (tag, payload) in [
            (b"phdr", &phdr[..]),
            (b"pbag", &pbag[..]),
            (b"pmod", &terminal_modulator[..]),
            (b"pgen", &pgen[..]),
            (b"inst", &inst[..]),
            (b"ibag", &ibag[..]),
            (b"imod", &terminal_modulator[..]),
            (b"igen", &igen[..]),
            (b"shdr", &shdr[..]),
        ] {
            pdta.extend(chunk(tag, payload));
        }

        let mut body = b"sfbk".to_vec();
        body.extend(chunk(b"LIST", &[&b"INFO"[..], &info].concat()));
        body.extend(chunk(b"LIST", &[&b"sdta"[..], &sdta].concat()));
        body.extend(chunk(b"LIST", &[&b"pdta"[..], &pdta].concat()));
        let mut file = b"RIFF".to_vec();
        file.extend_from_slice(&(body.len() as u32).to_le_bytes());
        file.extend(body);
        file
    }
}

fn chunk(tag: &[u8; 4], payload: &[u8]) -> Vec<u8> {
    let mut out = tag.to_vec();
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(payload);
    if payload.len() % 2 == 1 {
        out.push(0); // RIFF pad byte, outside the declared size
    }
    out
}

fn name20(name: &str) -> Vec<u8> {
    let mut out = name.as_bytes()[..name.len().min(19)].to_vec();
    out.resize(20, 0);
    out
}

fn sample_header(name: &str, bounds: [u32; 4], rate: u32, sample_type: u16) -> Vec<u8> {
    let mut out = name20(name);
    for value in bounds.into_iter().chain([rate]) {
        out.extend_from_slice(&value.to_le_bytes());
    }
    out.extend_from_slice(&[60, 0, 0, 0]); // original pitch, correction, link
    out.extend_from_slice(&sample_type.to_le_bytes());
    out
}

/// Bag and generator lists for a list of zone lists, with the bag index each
/// owner starts at. Both lists end in the terminal record the format requires,
/// and the returned indices end with the terminal owner's.
fn bags(owners: &[&Vec<Zone>]) -> (Vec<u16>, Vec<u8>, Vec<u8>) {
    let mut starts = Vec::new();
    let mut bag = Vec::new();
    let mut gen = Vec::new();
    let (mut bag_index, mut gen_index) = (0_u16, 0_u16);
    for zones in owners {
        starts.push(bag_index);
        for zone in zones.iter() {
            bag.extend_from_slice(&gen_index.to_le_bytes());
            bag.extend_from_slice(&0_u16.to_le_bytes());
            for &(kind, value) in zone {
                gen.extend_from_slice(&kind.to_le_bytes());
                gen.extend_from_slice(&value.to_le_bytes());
                gen_index += 1;
            }
            bag_index += 1;
        }
    }
    starts.push(bag_index);
    bag.extend_from_slice(&gen_index.to_le_bytes());
    bag.extend_from_slice(&0_u16.to_le_bytes());
    gen.extend_from_slice(&[0; 4]);
    (starts, bag, gen)
}

/// Selects `program` (bank, patch) on channel 0, holds `keys` for `blocks`
/// blocks of 64 frames, releases them and renders as long again. Left and
/// right, one block after the other.
pub(crate) fn render(
    font: &Arc<SoundFont>,
    program: (i32, i32),
    keys: &[i32],
    velocity: i32,
    blocks: usize,
) -> Vec<f32> {
    let mut synth = Synthesizer::new(font, &SynthesizerSettings::new(44100)).unwrap();
    synth.process_midi_message(0, 0xB0, 0x00, program.0);
    synth.process_midi_message(0, 0xC0, program.1, 0);
    for &key in keys {
        synth.note_on(0, key, velocity);
    }
    let (mut left, mut right) = (vec![0_f32; 64], vec![0_f32; 64]);
    let mut out = Vec::new();
    for block in 0..blocks * 2 {
        if block == blocks {
            for &key in keys {
                synth.note_off(0, key);
            }
        }
        synth.render(&mut left, &mut right);
        out.extend_from_slice(&left);
        out.extend_from_slice(&right);
    }
    out
}
