pub trait Value: Copy {}

impl<T: Copy> Value for T {}

pub trait Coord: Copy {
    type Item: Value;

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self;

    fn x(&self) -> Self::Item;

    fn y(&self) -> Self::Item;
}
