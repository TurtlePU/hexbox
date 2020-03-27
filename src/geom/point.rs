use derive_more::{Add, Sub};

#[derive(Clone, Copy, Default, Add, Sub)]
pub struct Point<T>(T, T);

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self { Point(x, y) }
}
