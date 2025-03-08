use crate::chunk_type::ChunkType;

#[allow(dead_code)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}
