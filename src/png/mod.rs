// src/png/mod.rs

use std::{fmt, fs};
use std::io::{BufReader, Read, Write};
use std::path::Path;

use crate::chunks::chunk::Chunk;
use crate::error::{Error, Result};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Png {
    chunks: Vec<Chunk>
}

#[allow(dead_code)]
impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
    
    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        Self { chunks }
    }

    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    pub fn remove_first_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        if let Some(index) = self.chunks.iter().position(|chunk| chunk.chunk_type().to_string() == chunk_type) {
            Ok(self.chunks.remove(index))
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Chunk type {} not found", chunk_type),
            )))
        }
    }
    
    pub fn header(&self) -> &[u8; 8] {  
        &Self::STANDARD_HEADER
    }

    pub fn chunks(&self) -> &Vec<Chunk> {
        &self.chunks
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        self.chunks.iter().find(|chunk| chunk.chunk_type().to_string() == chunk_type)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.header());
        for chunk in &self.chunks {
            bytes.extend_from_slice(&chunk.as_bytes());
        }
        bytes   
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let bytes = fs::read(path)?;
        Self::try_from(bytes.as_ref())
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let bytes = self.as_bytes();
        let mut file = fs::File::create(path)?;
        file.write_all(&bytes)?;
        Ok(())
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = Error;
    fn try_from(bytes: &[u8]) -> Result<Png> {
        let mut reader = BufReader::new(bytes);
        let mut header = [0u8; 8];
        reader.read_exact(&mut header)?;
        if header != Png::STANDARD_HEADER {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid PNG header",
            )));
        }
        let mut chunks = Vec::new();
        loop {
            let mut length_bytes = [0u8; 4];
            match reader.read_exact(&mut length_bytes) {
                Ok(()) => {},
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::UnexpectedEof {
                        break;
                    }
                    else {
                        return Err(Box::new(e));
                    }
                }
            }
            let length = u32::from_be_bytes(length_bytes);
            let chunk_total_length = 4 + (length as usize) + 4;
            let mut chunk_bytes = vec![0u8; chunk_total_length];
            reader.read_exact(&mut chunk_bytes)?;
            let mut full_chunk = Vec::with_capacity(4 + chunk_total_length);
            full_chunk.extend_from_slice(&length_bytes);
            full_chunk.extend_from_slice(&chunk_bytes);
            let chunk = Chunk::try_from(full_chunk.as_slice())?;
            chunks.push(chunk);
        }
        Ok(Png { chunks })
    }
}

impl fmt::Display for Png {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PNG File")?;
        writeln!(f, "Header: {:?}", Png::STANDARD_HEADER)?;
        writeln!(f, "Chunks:")?;
        for chunk in &self.chunks {
            writeln!(f, "{}", chunk)?;
        }
        Ok(())
    }
}