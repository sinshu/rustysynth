use std::error;
use std::io;

use super::binary_reader;

pub(crate) struct Generator
{
    pub(crate) generator_type: u16,
    pub(crate) value: u16,
}

impl Generator
{
    fn new<R: io::Read>(reader: &mut R) -> Result<Self, Box<dyn error::Error>>
    {
        let result = binary_reader::read_u16(reader);
        let generator_type = match result
        {
            Ok(value) => value,
            Err(error) => return Err(Box::new(error)),
        };

        let result = binary_reader::read_u16(reader);
        let value = match result
        {
            Ok(value) => value,
            Err(error) => return Err(Box::new(error)),
        };

        Ok(Generator
        {
            generator_type: generator_type,
            value: value,
        })
    }
}

pub(crate) fn read_from_chunk<R: io::Read>(reader: &mut R, size: i32) -> Result<Vec<Generator>, Box<dyn error::Error>>
{
    if size % 4 != 0
    {
        return Err(format!("The generator list is invalid.").into());
    }

    let count = size / 4 - 1;

    let mut generators: Vec<Generator> = Vec::new();
    for _i in 0..count
    {
        let result = Generator::new(reader);
        match result
        {
            Ok(value) => generators.push(value),
            Err(error) => return Err(error),
        }
    }

    // The last one is the terminator.
    let result = Generator::new(reader);
    match result
    {
        Ok(_value) => (),
        Err(error) => return Err(error),
    };

    Ok(generators)
}
