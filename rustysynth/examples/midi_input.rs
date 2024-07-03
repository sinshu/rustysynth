use anyhow::{anyhow, Result};
use clap::Parser;
use itertools::Itertools;
use midir::{Ignore, MidiInput};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tinyaudio::prelude::*;

use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};

#[derive(Debug, Parser)]
struct Opts {
    #[arg(short, long, help = "List MIDI ports and exit")]
    list: bool,
    #[arg(
        short,
        long,
        default_value = "0",
        help = "MIDI port (use --list to show ports)"
    )]
    port: usize,
    #[arg(short, long, default_value = "1.0", help = "Master volume")]
    volume: f32,
    #[arg(long, help = "Channel override")]
    channel: Option<i32>,
    #[arg(long, help = "Program change")]
    program: Option<i32>,
    #[arg(long, help = "Soundfont file")]
    sf2: Option<PathBuf>,
}

fn setup_synth(sf2: &Path, sample_rate: i32) -> Result<Arc<Mutex<Synthesizer>>> {
    let mut sf2 = File::open(sf2)?;
    let soundfont = Arc::new(SoundFont::new(&mut sf2)?);
    let settings = SynthesizerSettings::new(sample_rate);
    let synthesizer = Synthesizer::new(&soundfont, &settings)?;
    Ok(Arc::new(Mutex::new(synthesizer)))
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    let mut midi = MidiInput::new("rustysynth")?;
    midi.ignore(Ignore::None);

    // Get the list of available input ports.
    let ports = midi.ports();
    if opts.list {
        // List ports for the user.
        for (i, p) in ports.iter().enumerate() {
            println!("{i}: {}", midi.port_name(p)?);
        }
        return Ok(());
    }

    // Set our desired audio output parameters.
    let audio = OutputDeviceParameters {
        channels_count: 2,
        sample_rate: 44100,
        channel_sample_count: 1024,
    };

    // Setup the soundfont synthesizer.
    // If you don't have an SF2 file around, check here:
    // https://archive.org/download/free-soundfonts-sf2-2019-04
    let synth = if let Some(sf2) = &opts.sf2 {
        setup_synth(sf2, audio.sample_rate as i32)?
    } else {
        return Err(anyhow!("Missing sf2 file"));
    };

    // Set the master volume and perform a program change if asked.
    synth.lock().unwrap().set_master_volume(opts.volume);
    if let Some(program) = opts.program {
        synth.lock().unwrap().process_midi_message(
            opts.channel.map(|ch| ch - 1).unwrap_or(0),
            0xC0,
            program - 1,
            0,
        );
    }

    let port = &ports[opts.port];
    println!("Connecting to {}", midi.port_name(port)?);

    // Connect to the MIDI input port and send the messages to the synthesizer.
    let _conn = midi
        .connect(
            port,
            "rustysynth-input",
            {
                let synth = Arc::clone(&synth);
                let chovr = opts.channel.clone().map(|ch| ch - 1);
                move |time, message, _| {
                    println!("{time:>10}: {message:02x?}");
                    let channel = chovr.unwrap_or(message[0] as i32 & 0x0F);
                    let command = message[0] as i32 & 0xF0;
                    synth.lock().unwrap().process_midi_message(
                        channel,
                        command,
                        *message.get(1).unwrap_or(&0) as i32,
                        *message.get(2).unwrap_or(&0) as i32,
                    );
                }
            },
            (),
        )
        .unwrap();

    // Setup an audio callback to render synthesizer output.
    let _audev = run_output_device(audio, {
        let mut left = vec![0f32; audio.channel_sample_count];
        let mut right = vec![0f32; audio.channel_sample_count];
        move |data| {
            synth
                .lock()
                .unwrap()
                .render(left.as_mut_slice(), right.as_mut_slice());
            for (i, value) in left.iter().interleave(right.iter()).enumerate() {
                data[i] = *value;
            }
        }
    })
    .unwrap();

    println!("Press CTRL-C to quit.");
    loop {
        // Wait forever.
        std::thread::sleep(Duration::from_secs(1));
    }
}
