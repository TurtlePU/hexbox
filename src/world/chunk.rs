use crate::pos::ChunkPos;
use super::Block;

pub const CHUNK_SIZE: usize = 16;

pub struct Chunk {
    pos: ChunkPos,
    blocks: [[Block; CHUNK_SIZE]; CHUNK_SIZE],
}
