#![allow(dead_code)]

use std::io;
use std::io::Read;

use crate::binary_reader::BinaryReader;

#[non_exhaustive]
pub struct SoundFontVersion {
    pub(crate) major: i16,
    pub(crate) minor: i16,
}

impl SoundFontVersion {
    pub(crate) fn default() -> Self {
        Self { major: 0, minor: 0 }
    }

    pub(crate) fn new<R: Read>(reader: &mut R) -> Result<Self, io::Error> {
        let major = BinaryReader::read_i16(reader)?;
        let minor = BinaryReader::read_i16(reader)?;

        Ok(Self { major, minor })
    }

    pub fn get_major(&self) -> i32 {
        self.major as i32
    }

    pub fn get_minor(&self) -> i32 {
        self.minor as i32
    }
}
