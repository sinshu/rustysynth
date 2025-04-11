#![allow(unused_imports)]

use rustysynth::SoundFont;
use rustysynth::SoundFontError;
use std::fs::File;
use std::path::PathBuf;

#[test]
fn soundfont3_load_test() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.push("MuseScore_General.sf3");
    let mut file = File::open(&path).unwrap();
    let result = SoundFont::new(&mut file);
    match result {
        Ok(_) => panic!(),
        Err(err) => match err {
            SoundFontError::UnsupportedSampleFormat => {}
            _ => panic!(),
        },
    }
}
