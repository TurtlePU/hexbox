pub mod block;
pub mod chunk;

pub use block::Block;
pub use chunk::Chunk;

use std::collections::HashMap;
use crate::pos::ChunkPos;

pub struct World(HashMap<ChunkPos, Chunk>);
