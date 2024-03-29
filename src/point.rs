use crate::coord::{Coord, Primitive};
use num;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point<T: Primitive> {
    x: T,
    y: T,
}

impl<T: Primitive> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// # Examples
    /// ```
    /// # use coord::Point;
    /// assert_eq!(4, Point::new(4, 8).x());
    /// ```
    pub fn x(self) -> T {
        self.x
    }

    /// # Examples
    /// ```
    /// # use coord::Point;
    /// assert_eq!(8, Point::new(4, 8).y());
    /// ```
    pub fn y(self) -> T {
        self.y
    }

    /// # Examples
    /// ```
    /// # use coord::Point;
    /// assert_eq!(Point::new(0, 0), Point::zero());
    /// ```
    pub fn zero() -> Self
    where
        T: num::Zero,
    {
        Self::new(T::zero(), T::zero())
    }

    /// # Examples
    /// ```
    /// # use coord::Point;
    /// assert!(Point::new(0, 0).is_zero());
    /// ```
    pub fn is_zero(&self) -> bool
    where
        T: num::Zero,
    {
        self.x().is_zero() && self.y().is_zero()
    }
}

impl<T: Primitive> Coord for Point<T> {
    type Item = T;

    /// # Examples
    /// ```
    /// # use coord::Point;
    /// # use coord::Coord;
    /// assert_eq!(Point::new(6, 12), Point::from_x_y(6, 12));
    /// ```
    fn from_x_y(x: T, y: T) -> Self {
        Self::new(x, y)
    }

    fn x(self) -> T {
        self.x()
    }

    fn y(self) -> T {
        self.y()
    }
}

impl<T: Primitive + fmt::Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}
