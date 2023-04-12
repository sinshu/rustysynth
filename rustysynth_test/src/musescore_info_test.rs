#![allow(unused_imports)]

use rustysynth::SoundFont;
use std::fs::File;
use std::path::PathBuf;

#[test]
fn soundfont_info() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.push("GeneralUser GS MuseScore v1.442.sf2");
    let mut file = File::open(&path).unwrap();
    let sf = SoundFont::new(&mut file).unwrap();
    let info = sf.get_info();

    assert_eq!(info.get_version().get_major(), 2);
    assert_eq!(info.get_version().get_minor(), 1);
    assert_eq!(info.get_target_sound_engine(), "E-mu 10K2");
    assert_eq!(
        info.get_bank_name(),
        "GeneralUser GS MuseScore version 1.442"
    );
    assert_eq!(info.get_rom_name(), "");
    assert_eq!(info.get_rom_version().get_major(), 0);
    assert_eq!(info.get_rom_version().get_minor(), 0);
    assert_eq!(info.get_creation_date(), "");
    assert_eq!(info.get_author(), "S. Christian Collins");
    assert_eq!(info.get_target_product(), "");
    assert_eq!(info.get_copyright(), "2012 by S. Christian Collins");
    assert_eq!(info.get_comments().len(), 2207);
    assert_eq!(info.get_tools(), ":SFEDT v1.10:SFEDT v1.36:");

    assert_eq!(sf.get_wave_data().len(), 15513098);

    let mut sum: i32 = 0;
    for value in sf.get_wave_data().iter() {
        sum += *value as i32;
    }
    assert_eq!(sum, 101035585)
}
