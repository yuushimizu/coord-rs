use crate::coord::{Coord, Value};

impl<T: Value, C: Coord<T>> Coord<(T,)> for (C,) {
    fn from_x_y(x: (T,), y: (T,)) -> Self {
        (C::from_x_y(x.0, y.0),)
    }

    /// # Examples
    /// ```
    /// use coord::Coord;
    /// use coord::Point;
    /// assert_eq!((10,), (Point::new(10, 20),).x());
    /// ```
    fn x(&self) -> (T,) {
        (self.0.x(),)
    }

    /// # Examples
    /// ```
    /// use coord::Coord;
    /// use coord::Point;
    /// assert_eq!((20,), (Point::new(10, 20),).y());
    /// ```
    fn y(&self) -> (T,) {
        (self.0.y(),)
    }
}

impl<T0: Value, T1: Value, C0: Coord<T0>, C1: Coord<T1>> Coord<(T0, T1)> for (C0, C1) {
    fn from_x_y(x: (T0, T1), y: (T0, T1)) -> Self {
        (C0::from_x_y(x.0, y.0), C1::from_x_y(x.1, y.1))
    }

    fn x(&self) -> (T0, T1) {
        (self.0.x(), self.1.x())
    }

    fn y(&self) -> (T0, T1) {
        (self.0.y(), self.1.y())
    }
}

impl<T0: Value, T1: Value, T2: Value, C0: Coord<T0>, C1: Coord<T1>, C2: Coord<T2>>
    Coord<(T0, T1, T2)> for (C0, C1, C2)
{
    fn from_x_y(x: (T0, T1, T2), y: (T0, T1, T2)) -> Self {
        (
            C0::from_x_y(x.0, y.0),
            C1::from_x_y(x.1, y.1),
            C2::from_x_y(x.2, y.2),
        )
    }

    fn x(&self) -> (T0, T1, T2) {
        (self.0.x(), self.1.x(), self.2.x())
    }

    fn y(&self) -> (T0, T1, T2) {
        (self.0.y(), self.1.y(), self.2.y())
    }
}
