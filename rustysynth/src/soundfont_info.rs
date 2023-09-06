#![allow(dead_code)]

use std::io::Read;

use crate::binary_reader::BinaryReader;
use crate::error::SoundFontError;
use crate::four_cc::FourCC;
use crate::read_counter::ReadCounter;
use crate::soundfont_version::SoundFontVersion;

/// The information of a SoundFont.
#[non_exhaustive]
pub struct SoundFontInfo {
    pub(crate) version: SoundFontVersion,
    pub(crate) target_sound_engine: String,
    pub(crate) bank_name: String,
    pub(crate) rom_name: String,
    pub(crate) rom_version: SoundFontVersion,
    pub(crate) creation_date: String,
    pub(crate) author: String,
    pub(crate) target_product: String,
    pub(crate) copyright: String,
    pub(crate) comments: String,
    pub(crate) tools: String,
}

impl SoundFontInfo {
    pub(crate) fn new<R: Read>(reader: &mut R) -> Result<Self, SoundFontError> {
        let chunk_id = BinaryReader::read_four_cc(reader)?;
        if &chunk_id != b"LIST" {
            return Err(SoundFontError::ListChunkNotFound);
        }

        let end = BinaryReader::read_i32(reader)? as usize;
        let reader = &mut ReadCounter::new(reader);

        let list_type = BinaryReader::read_four_cc(reader)?;
        if &list_type != b"INFO" {
            return Err(SoundFontError::InvalidListChunkType {
                expected: FourCC::from_bytes(*b"INFO"),
                actual: list_type,
            });
        }

        let mut version: Option<SoundFontVersion> = None;
        let mut target_sound_engine: Option<String> = None;
        let mut bank_name: Option<String> = None;
        let mut rom_name: Option<String> = None;
        let mut rom_version: Option<SoundFontVersion> = None;
        let mut creation_date: Option<String> = None;
        let mut author: Option<String> = None;
        let mut target_product: Option<String> = None;
        let mut copyright: Option<String> = None;
        let mut comments: Option<String> = None;
        let mut tools: Option<String> = None;

        while reader.bytes_read() < end {
            let id = BinaryReader::read_four_cc(reader)?;
            let size = BinaryReader::read_i32(reader)? as usize;

            match id.as_bytes() {
                b"ifil" => version = Some(SoundFontVersion::new(reader)?),
                b"isng" => {
                    target_sound_engine =
                        Some(BinaryReader::read_fixed_length_string(reader, size)?)
                }
                b"INAM" => bank_name = Some(BinaryReader::read_fixed_length_string(reader, size)?),
                b"irom" => rom_name = Some(BinaryReader::read_fixed_length_string(reader, size)?),
                b"iver" => rom_version = Some(SoundFontVersion::new(reader)?),
                b"ICRD" => {
                    creation_date = Some(BinaryReader::read_fixed_length_string(reader, size)?)
                }
                b"IENG" => author = Some(BinaryReader::read_fixed_length_string(reader, size)?),
                b"IPRD" => {
                    target_product = Some(BinaryReader::read_fixed_length_string(reader, size)?)
                }
                b"ICOP" => copyright = Some(BinaryReader::read_fixed_length_string(reader, size)?),
                b"ICMT" => comments = Some(BinaryReader::read_fixed_length_string(reader, size)?),
                b"ISFT" => tools = Some(BinaryReader::read_fixed_length_string(reader, size)?),
                _ => return Err(SoundFontError::ListContainsUnknownId(id)),
            }
        }

        let version = version.unwrap_or_else(SoundFontVersion::default);
        let target_sound_engine = target_sound_engine.unwrap_or_default();
        let bank_name = bank_name.unwrap_or_default();
        let rom_name = rom_name.unwrap_or_default();
        let rom_version = rom_version.unwrap_or_else(SoundFontVersion::default);
        let creation_date = creation_date.unwrap_or_default();
        let author = author.unwrap_or_default();
        let target_product = target_product.unwrap_or_default();
        let copyright = copyright.unwrap_or_default();
        let comments = comments.unwrap_or_default();
        let tools = tools.unwrap_or_default();

        Ok(Self {
            version,
            target_sound_engine,
            bank_name,
            rom_name,
            rom_version,
            creation_date,
            author,
            target_product,
            copyright,
            comments,
            tools,
        })
    }

    /// Gets the version of the SoundFont.
    pub fn get_version(&self) -> &SoundFontVersion {
        &self.version
    }

    /// Gets the target sound engine of the SoundFont.
    pub fn get_target_sound_engine(&self) -> &str {
        &self.target_sound_engine
    }

    /// Gets the bank name of the SoundFont.
    pub fn get_bank_name(&self) -> &str {
        &self.bank_name
    }

    /// Gets the ROM name of the SoundFont.
    pub fn get_rom_name(&self) -> &str {
        &self.rom_name
    }

    /// Gets the ROM version of the SoundFont.
    pub fn get_rom_version(&self) -> &SoundFontVersion {
        &self.rom_version
    }

    /// Gets the creation date of the SoundFont.
    pub fn get_creation_date(&self) -> &str {
        &self.creation_date
    }

    /// Gets the auther of the SoundFont.
    pub fn get_author(&self) -> &str {
        &self.author
    }

    /// Gets the target product of the SoundFont.
    pub fn get_target_product(&self) -> &str {
        &self.target_product
    }

    /// Gets the copyright message for the SoundFont.
    pub fn get_copyright(&self) -> &str {
        &self.copyright
    }

    /// Gets the comments for the SoundFont.
    pub fn get_comments(&self) -> &str {
        &self.comments
    }

    /// Gets the tools used to create the SoundFont.
    pub fn get_tools(&self) -> &str {
        &self.tools
    }
}
