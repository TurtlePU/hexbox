use std::ops::Deref;
use derive_more::{Add, Sub};
use super::point::Point;
use super::hex_pos::HexPos;

#[derive(Clone, Copy, Default, Add, Sub)]
pub struct CubicPos(Point<i64>);

impl CubicPos {
    fn new(x: i64, y: i64) -> Self { Self(Point::new(x, y)) }
    fn up() -> Self { Self::new(0, 1) }
    fn left() -> Self { Self::new(-1, 0) }
    fn right() -> Self { Self::new(1, -1) }
}

impl Deref for CubicPos {
    type Target = Point<i64>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl HexPos for CubicPos {
    fn above(&self) -> Self { *self + Self::up() }
    fn below(&self) -> Self { *self - Self::up() }
    fn left_arm(&self) -> Self { *self - Self::right() }
    fn right_arm(&self) -> Self { *self - Self::left() }
    fn left(&self) -> Self { *self + Self::left() }
    fn right(&self) -> Self { *self + Self::right() }
}
