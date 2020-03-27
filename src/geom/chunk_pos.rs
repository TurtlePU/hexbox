use crate::world::chunk::CHUNK_SIZE;
use super::offset_pos::OffsetPos;

pub struct ChunkPos(OffsetPos);

impl ChunkPos {
    pub fn row(&self) -> isize { self.0.row() }
    pub fn col(&self) -> isize { self.0.col() }
}

impl From<OffsetPos> for ChunkPos {
    fn from(offset: OffsetPos) -> ChunkPos {
        const SIZE: isize = CHUNK_SIZE as isize;
        ChunkPos(OffsetPos::new(offset.row() / SIZE, offset.col() / SIZE))
    }
}
