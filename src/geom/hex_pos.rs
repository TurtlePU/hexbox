pub trait HexPos {
    fn above(&self) -> Self;
    fn below(&self) -> Self;
    fn left_arm(&self) -> Self;
    fn right_arm(&self) -> Self;
    fn left(&self) -> Self;
    fn right(&self) -> Self;
}
