use crate::coord::{Axis, Coord};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T: Axis> {
    x: T,
    y: T,
}

impl<T: Axis> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// # Examples
    /// ```
    /// use coord::Point;
    /// assert_eq!(4, Point::new(4, 8).x());
    /// ```
    pub fn x(&self) -> T {
        self.x
    }

    /// # Examples
    /// ```
    /// use coord::Point;
    /// assert_eq!(8, Point::new(4, 8).y());
    /// ```
    pub fn y(&self) -> T {
        self.y
    }
}

impl<T: Axis> Coord<T> for Point<T> {
    /// # Examples
    /// ```
    /// use coord::Point;
    /// use coord::Coord;
    /// assert_eq!(Point::new(6, 12), Point::from_x_y(6, 12));
    /// ```
    fn from_x_y(x: T, y: T) -> Self {
        Self::new(x, y)
    }

    fn x(&self) -> T {
        self.x()
    }

    fn y(&self) -> T {
        self.y()
    }
}
