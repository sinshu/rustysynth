#![allow(unused_imports)]

use std::fs::File;
use std::path::PathBuf;
use rustysynth::SoundFont;

#[test]
fn soundfont_info()
{
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.push("GeneralUser GS MuseScore v1.442.sf2");
    let mut file = File::open(&path).unwrap();
    let sf = SoundFont::new(&mut file).unwrap();

    assert_eq!(sf.info.version.major, 2);
    assert_eq!(sf.info.version.minor, 1);
    assert_eq!(sf.info.target_sound_engine.as_str(), "E-mu 10K2");
    assert_eq!(sf.info.bank_name.as_str(), "GeneralUser GS MuseScore version 1.442");
    assert_eq!(sf.info.rom_name.as_str(), "");
    assert_eq!(sf.info.rom_version.major, 0);
    assert_eq!(sf.info.rom_version.minor, 0);
    assert_eq!(sf.info.creation_date.as_str(), "");
    assert_eq!(sf.info.author.as_str(), "S. Christian Collins");
    assert_eq!(sf.info.target_product.as_str(), "");
    assert_eq!(sf.info.copyright.as_str(), "2012 by S. Christian Collins");
    assert_eq!(sf.info.comments.len(), 2207);
    assert_eq!(sf.info.tools.as_str(), ":SFEDT v1.10:SFEDT v1.36:");

    assert_eq!(sf.wave_data.len(), 15513098);

    let mut sum: i32 = 0;
    for value in sf.wave_data.iter()
    {
        sum += *value as i32;
    }
    assert_eq!(sum, 101035585)
}
