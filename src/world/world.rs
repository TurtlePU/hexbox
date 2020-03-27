use std::collections::HashMap;
use crate::geom::chunk_pos::ChunkPos;
use super::chunk::Chunk;

pub struct World(HashMap<ChunkPos, Chunk>);
