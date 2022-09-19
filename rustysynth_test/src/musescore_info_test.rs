use std::fs;
use std::path;

#[test]
fn soundfont_info()
{
    let mut path = path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.push("GeneralUser GS MuseScore v1.442.sf2");

    let mut file = fs::File::open(&path).unwrap();

    let sound_font = rustysynth::SoundFont::new(&mut file).unwrap();

    assert_eq!(sound_font.info.version.major, 2);
    assert_eq!(sound_font.info.version.minor, 1);
    assert_eq!(sound_font.info.target_sound_engine.as_str(), "E-mu 10K2");
    assert_eq!(sound_font.info.bank_name.as_str(), "GeneralUser GS MuseScore version 1.442");
    assert_eq!(sound_font.info.rom_name.as_str(), "");
    assert_eq!(sound_font.info.rom_version.major, 0);
    assert_eq!(sound_font.info.rom_version.minor, 0);
    assert_eq!(sound_font.info.creation_date.as_str(), "");
    assert_eq!(sound_font.info.author.as_str(), "S. Christian Collins");
    assert_eq!(sound_font.info.target_product.as_str(), "");
    assert_eq!(sound_font.info.copyright.as_str(), "2012 by S. Christian Collins");
    assert_eq!(sound_font.info.comments.len(), 2207);
    assert_eq!(sound_font.info.tools.as_str(), ":SFEDT v1.10:SFEDT v1.36:");
}
