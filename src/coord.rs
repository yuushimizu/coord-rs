pub trait Value: Copy {}

impl<T: Copy> Value for T {}

pub trait Coord<T: Value>: Copy {
    fn from_x_y(x: T, y: T) -> Self;

    fn x(&self) -> T;

    fn y(&self) -> T;
}
