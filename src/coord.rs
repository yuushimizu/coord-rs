pub trait Axis: Copy {}

impl<T: Copy> Axis for T {}

pub trait Coord<T: Axis>: Copy {
    fn from_x_y(x: T, y: T) -> Self;

    fn x(&self) -> T;

    fn y(&self) -> T;
}
