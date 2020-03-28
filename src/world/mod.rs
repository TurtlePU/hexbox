pub mod block;
pub mod chunk;

use std::collections::HashMap;
use crate::pos::chunk_pos::ChunkPos;
use chunk::Chunk;

pub struct World(HashMap<ChunkPos, Chunk>);
