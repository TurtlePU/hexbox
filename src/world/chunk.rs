use crate::geom::chunk_pos::ChunkPos;
use super::block::Block;

pub const CHUNK_SIZE: usize = 16;

pub struct Chunk(ChunkPos, [[Block; CHUNK_SIZE]; CHUNK_SIZE]);
