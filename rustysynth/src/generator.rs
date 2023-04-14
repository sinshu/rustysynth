#![allow(dead_code)]

use std::error::Error;
use std::io::Read;

use crate::binary_reader::BinaryReader;

#[derive(Clone, Copy)]
#[non_exhaustive]
pub(crate) struct Generator {
    pub(crate) generator_type: u16,
    pub(crate) value: u16,
}

impl Generator {
    fn new<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let generator_type = BinaryReader::read_u16(reader)?;
        let value = BinaryReader::read_u16(reader)?;

        Ok(Self {
            generator_type,
            value,
        })
    }

    pub(crate) fn read_from_chunk<R: Read>(
        reader: &mut R,
        size: i32,
    ) -> Result<Vec<Generator>, Box<dyn Error>> {
        if size % 4 != 0 {
            return Err("The generator list is invalid.".into());
        }

        let count = size / 4 - 1;

        let mut generators: Vec<Generator> = Vec::new();
        for _i in 0..count {
            generators.push(Generator::new(reader)?);
        }

        // The last one is the terminator.
        Generator::new(reader)?;

        Ok(generators)
    }
}
