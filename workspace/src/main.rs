use std::fs::File;
use std::path::Path;

fn main()
{
    println!("Hello, world!");

    let path = Path::new("TimGM6mb.sf2");
    let mut file = File::open(&path).unwrap();

    let sf = rustysynth::SoundFont::new(&mut file).unwrap();

    println!("Bank name is \"{}\"", sf.info.bank_name);
    println!("Bank name is \"{}\"", sf.info.target_sound_engine);
}
