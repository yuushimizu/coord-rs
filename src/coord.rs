pub trait Primitive: Copy {}

impl<T: Copy> Primitive for T {}

pub trait Coord: Copy {
    type Item: Primitive;

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self;

    fn x(self) -> Self::Item;

    fn y(self) -> Self::Item;
}
