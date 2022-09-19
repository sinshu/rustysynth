use std::fs;
use std::path;

#[test]
fn soundfont_info()
{
    let mut path = path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.push("TimGM6mb.sf2");

    let mut file = fs::File::open(&path).unwrap();

    let sound_font = rustysynth::SoundFont::new(&mut file).unwrap();

    assert_eq!(sound_font.info.version.major, 2);
    assert_eq!(sound_font.info.version.minor, 1);
    assert_eq!(sound_font.info.target_sound_engine.as_str(), "EMU8000");
    assert_eq!(sound_font.info.bank_name.as_str(), "TimGM6mb1.sf2");
    assert_eq!(sound_font.info.rom_name.as_str(), "");
    assert_eq!(sound_font.info.rom_version.major, 0);
    assert_eq!(sound_font.info.rom_version.minor, 0);
    assert_eq!(sound_font.info.creation_date.as_str(), "");
    assert_eq!(sound_font.info.author.as_str(), "");
    assert_eq!(sound_font.info.target_product.as_str(), "");
    assert_eq!(sound_font.info.copyright.as_str(), "");
    assert_eq!(sound_font.info.comments.as_str(), "");
    assert_eq!(sound_font.info.tools.as_str(), "Awave Studio v8.5");
}
