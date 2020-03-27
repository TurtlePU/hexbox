use super::cubic_pos::CubicPos;

pub struct OffsetPos(i64, i64);

impl OffsetPos {
    pub fn row(&self) -> i64 { self.0 }
    pub fn col(&self) -> i64 { self.1 }
}

impl From<CubicPos> for OffsetPos {
    fn from(cubic: CubicPos) -> OffsetPos {
        let x = cubic.x();
        OffsetPos(x, cubic.z() + (x - x & 1) / 2)
    }
}
