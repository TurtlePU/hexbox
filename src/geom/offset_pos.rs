use super::point::Point;
use super::hex_pos::HexPos;

pub struct OffsetPos(Point<i64>);

impl HexPos for OffsetPos {
    fn above(&self) -> Self { todo!() }
    fn below(&self) -> Self { todo!() }
    fn left_arm(&self) -> Self { todo!() }
    fn right_arm(&self) -> Self { todo!() }
    fn left(&self) -> Self { todo!() }
    fn right(&self) -> Self { todo!() }
}
