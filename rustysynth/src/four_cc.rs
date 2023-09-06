use std::fmt::{Debug, Display, Formatter, Result, Write};

#[derive(PartialEq, Eq)]
pub struct FourCC([u8; 4]);

impl FourCC {
    pub(crate) const fn from_bytes(mut bytes: [u8; 4]) -> Self {
        bytes[0] = replace_if_non_ascii(bytes[0]);
        bytes[1] = replace_if_non_ascii(bytes[1]);
        bytes[2] = replace_if_non_ascii(bytes[2]);
        bytes[3] = replace_if_non_ascii(bytes[3]);
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; 4] {
        &self.0
    }
}

impl Display for FourCC {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for byte in &self.0 {
            f.write_char(*byte as char)?;
        }
        Ok(())
    }
}

impl Debug for FourCC {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_char('"')?;
        for byte in &self.0 {
            f.write_char(*byte as char)?;
        }
        f.write_char('"')?;
        Ok(())
    }
}

impl PartialEq<[u8; 4]> for FourCC {
    fn eq(&self, other: &[u8; 4]) -> bool {
        &self.0 == other
    }
}

const fn is_ascii_graphic_or_space(byte: u8) -> bool {
    byte.is_ascii_graphic() || byte == b' '
}

const fn replace_if_non_ascii(byte: u8) -> u8 {
    if is_ascii_graphic_or_space(byte) {
        byte
    } else {
        b'?'
    }
}
