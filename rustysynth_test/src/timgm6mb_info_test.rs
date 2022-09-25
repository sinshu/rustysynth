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
    let info = sound_font.get_info();

    assert_eq!(info.get_version().get_major(), 2);
    assert_eq!(info.get_version().get_minor(), 1);
    assert_eq!(info.get_target_sound_engine(), "EMU8000");
    assert_eq!(info.get_bank_name(), "TimGM6mb1.sf2");
    assert_eq!(info.get_rom_name(), "");
    assert_eq!(info.get_rom_version().get_major(), 0);
    assert_eq!(info.get_rom_version().get_minor(), 0);
    assert_eq!(info.get_creation_date(), "");
    assert_eq!(info.get_author(), "");
    assert_eq!(info.get_target_product(), "");
    assert_eq!(info.get_copyright(), "");
    assert_eq!(info.get_comments(), "");
    assert_eq!(info.get_tools(), "Awave Studio v8.5");

    assert_eq!(sound_font.get_wave_data().len(), 2882168);

    let mut sum: i32 = 0;
    for value in sound_font.get_wave_data().iter()
    {
        sum += *value as i32;
    }
    assert_eq!(sum, 713099516)
}
