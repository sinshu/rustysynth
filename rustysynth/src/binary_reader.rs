use std::error;
use std::io;
use std::str;

pub(crate) fn read_i16<R: io::Read>(reader: &mut R) -> Result<i16, io::Error>
{
    let mut data: [u8; 2] = [0; 2];
    reader.read_exact(&mut data)?;
    Ok(i16::from_le_bytes(data))
}

pub(crate) fn read_u16<R: io::Read>(reader: &mut R) -> Result<u16, io::Error>
{
    let mut data: [u8; 2] = [0; 2];
    reader.read_exact(&mut data)?;
    Ok(u16::from_le_bytes(data))
}

pub(crate) fn read_i32<R: io::Read>(reader: &mut R) -> Result<i32, io::Error>
{
    let mut data: [u8; 4] = [0; 4];
    reader.read_exact(&mut data)?;
    Ok(i32::from_le_bytes(data))
}

pub(crate) fn read_four_cc<R: io::Read>(reader: &mut R) -> Result<String, Box<dyn error::Error>>
{
    let mut data: [u8; 4] = [0; 4];
    match reader.read_exact(&mut data)
    {
        Ok(()) => (),
        Err(error) => return Err(Box::new(error)),
    };

    for i in 0..4
    {
        let value = data[i];
        if !(32 <= value && value <= 126)
        {
            data[i] = 63; // '?'
        }
    }
    
    match str::from_utf8(&data)
    {
        Ok(value) => Ok(value.to_string()),
        Err(error) => Err(Box::new(error)),
    }
}

pub(crate) fn read_fixed_length_string<R: io::Read>(reader: &mut R, length: i32) -> Result<String, Box<dyn error::Error>>
{
    let mut data: Vec<u8> = vec![0; length as usize];
    match reader.read_exact(&mut data)
    {
        Ok(()) => (),
        Err(error) => return Err(Box::new(error)),
    }

    match str::from_utf8(&data)
    {
        Ok(value) => Ok(value.to_string()),
        Err(error) => Err(Box::new(error)),
    }
}
