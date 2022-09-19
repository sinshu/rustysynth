use std::fs::File;
use std::path::Path;

fn main()
{
    println!("Hello, world!");

    let path = Path::new("TimGM6mb.sf2");
    let mut file = File::open(&path).unwrap();

    let test = rustysynth::binary_reader::read_four_cc(&mut file).unwrap();

    println!("Header is \"{}\"!", test);
}
