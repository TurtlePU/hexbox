use derive_more::{Add, Sub};
use super::OffsetPos;
//use super::Direction;

#[derive(Clone, Copy, Add, Sub)]
pub struct CubicPos {
    x: isize,
    z: isize,
}

//use Direction::*;

impl CubicPos {
    pub fn x(&self) -> isize { self.x }
    pub fn z(&self) -> isize { self.z }
//    pub fn y(&self) -> i64 { -self.0 - self.1 }
//
//    pub fn go(&self, to: Direction) -> CubicPos {
//        let d = match to {
//            Up   => (0, -1), Right => ( 1, 0), Low  => (-1,  1),
//            Down => (0,  1), Left  => (-1, 0), High => ( 1, -1),
//        };
//        *self + CubicPos(d.0, d.1)
//    }
//
//    pub fn abs(&self) -> i64 {
//        vec![self.x(), self.y(), self.z()]
//            .into_iter()
//            .map(|x| x.abs())
//            .max().unwrap()
//    }
}

impl From<OffsetPos> for CubicPos {
    fn from(offset: OffsetPos) -> CubicPos {
        let col = offset.col();
        CubicPos { x: col, z: offset.row() - (col - col & 1) / 2 }
    }
}
