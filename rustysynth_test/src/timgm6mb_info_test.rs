#![allow(unused_imports)]

use std::fs::File;
use std::path::PathBuf;
use rustysynth::SoundFont;

#[test]
fn soundfont_info()
{
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.push("TimGM6mb.sf2");

    let mut file = File::open(&path).unwrap();

    let sound_font = SoundFont::new(&mut file).unwrap();

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

    assert_eq!(sound_font.wave_data.len(), 2882168);

    let mut sum: i32 = 0;
    for value in sound_font.wave_data.iter()
    {
        sum += *value as i32;
    }
    assert_eq!(sum, 713099516)
}
