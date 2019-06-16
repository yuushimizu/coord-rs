pub trait Primimtive: Copy {}

impl<T: Copy> Primimtive for T {}

pub trait Coord: Copy {
    type Item: Primimtive;

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self;

    fn x(&self) -> Self::Item;

    fn y(&self) -> Self::Item;
}
