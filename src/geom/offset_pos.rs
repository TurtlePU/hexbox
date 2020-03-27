use crate::world::chunk::CHUNK_SIZE;
use super::chunk_pos::ChunkPos;
use super::cubic_pos::CubicPos;

pub struct OffsetPos {
    row: isize,
    col: isize,
}

impl OffsetPos {
    pub fn new(row: isize, col: isize) -> OffsetPos {
        OffsetPos { row, col }
    }

    pub fn row(&self) -> isize { self.row }
    pub fn col(&self) -> isize { self.col }
}

impl From<CubicPos> for OffsetPos {
    fn from(cubic: CubicPos) -> OffsetPos {
        let x = cubic.x();
        OffsetPos::new(x, cubic.z() + (x - x & 1) / 2)
    }
}

impl From<ChunkPos> for OffsetPos {
    fn from(chunk: ChunkPos) -> OffsetPos {
        const SIZE: isize = CHUNK_SIZE as isize;
        OffsetPos::new(chunk.row() * SIZE, chunk.col() * SIZE)
    }
}
