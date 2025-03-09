use crate::chunks::chunk_type::ChunkType;
use crate::utils::crc::crc32;
use crate::{Error, Result};
use std::convert::TryFrom;
use std::fmt;
use std::io::{BufReader, Read};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

#[allow(dead_code)]
impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let length = data.len() as u32;
        let mut crc_input = Vec::new();
        crc_input.extend_from_slice(&chunk_type.bytes());
        crc_input.extend_from_slice(&data);
        let crc = crc32(&crc_input);
        Chunk {
            length,
            chunk_type,
            data,
            crc,
        }   
    }

    pub fn length(&self) -> u32 {
        self.length 
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        String::from_utf8(self.data.clone()).map_err(|e| Box::new(e) as Error)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(12 + self.data.len());
        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes.extend_from_slice(&self.chunk_type.bytes());
        bytes.extend_from_slice(&self.data);
        bytes.extend_from_slice(&self.crc.to_be_bytes());
        bytes
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;
    fn try_from(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 12 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Byte Slice too short to be a valid chunk",
            )));
        }
        let mut reader = BufReader::new(bytes);
        let mut length_bytes = [0u8; 4];
        reader.read_exact(&mut length_bytes)?;
        let length = u32::from_be_bytes(length_bytes);

        if bytes.len() != 4 + 4 + length as usize + 4 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Byte Slice Length does not match the Length Field",
            )));
        }
        let mut chunk_type_bytes = [0u8; 4];
        reader.read_exact(&mut chunk_type_bytes)?;
        let chunk_type = ChunkType::try_from(chunk_type_bytes)?;

        let mut data = vec![0u8; length as usize];
        reader.read_exact(&mut data)?;

        let mut crc_bytes = [0u8; 4];
        reader.read_exact(&mut crc_bytes)?;
        let crc_extracted = u32::from_be_bytes(crc_bytes);

        let mut crc_input = Vec::new();
        crc_input.extend_from_slice(&chunk_type.bytes());
        crc_input.extend_from_slice(&data);
        let crc_computed = crc32(&crc_input);

        if crc_extracted != crc_computed {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("CRC mismatch: computed = {:X}, extracted = {:X}", crc_computed, crc_extracted),
            )));
        }

        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc: crc_extracted,
        })
    }   
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{")?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}")?;
        Ok(())
    }
}