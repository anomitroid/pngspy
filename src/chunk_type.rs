use crate::{Error, Result};
use std::{fmt, str::FromStr};
use std::convert::TryFrom;

#[allow(dead_code)]
pub struct ChunkType {
    pub bytes: [u8; 4],
}

#[allow(dead_code)]
impl ChunkType {
    pub fn is_valid_byte(byte: u8) -> bool {
        byte.is_ascii_alphabetic()
    }

    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub fn is_critical(&self) -> bool {
        self.bytes[0].is_ascii_uppercase() && Self::is_valid_byte(self.bytes[0])
    }

    pub fn is_public(&self) -> bool {
        self.bytes[1].is_ascii_uppercase() && Self::is_valid_byte(self.bytes[1])
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2].is_ascii_uppercase() && Self::is_valid_byte(self.bytes[2])
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[3].is_ascii_lowercase() && Self::is_valid_byte(self.bytes[3])
    }

    pub fn is_valid(&self) -> bool {
        Self::is_valid_byte(self.bytes[0]) &&
        Self::is_valid_byte(self.bytes[1]) &&
        Self::is_valid_byte(self.bytes[2]) &&
        Self::is_valid_byte(self.bytes[3]) &&
        self.is_reserved_bit_valid()
    }
}

impl FromStr for ChunkType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 4 {
            return Err("Chunk type must be 4 characters long".into());
        }

        let bytes = s.as_bytes();
        for &b in bytes {
            if !Self::is_valid_byte(b) {
                return Err("Invalid byte".into());
            }
        }

        let array: [u8; 4] = bytes.try_into().unwrap();
        Ok(ChunkType { bytes: array })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8(self.bytes.to_vec()).unwrap())
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        Ok(ChunkType { bytes })
    }
}