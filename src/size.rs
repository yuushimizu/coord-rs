use crate::coord::{Coord, Value};
use std::fmt;
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size<T: Value> {
    width: T,
    height: T,
}

impl<T: Value> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    /// # Examples
    /// ```
    /// # use coord::Size;
    /// assert_eq!(10, Size::new(10, 20).width());
    /// ```
    pub fn width(&self) -> T {
        self.width
    }

    /// # Examples
    /// ```
    /// # use coord::Size;
    /// assert_eq!(20, Size::new(10, 20).height());
    /// ```
    pub fn height(&self) -> T {
        self.height
    }

    /// # Examples
    /// ```
    /// # use coord::Size;
    /// assert_eq!(12, Size::new(3, 4).area());
    /// ```
    pub fn area(&self) -> <T as Mul<T>>::Output
    where
        T: Mul<T>,
    {
        self.width() * self.height()
    }
}

impl<T: Value> Coord for Size<T> {
    type Item = T;

    /// # Examples
    /// ```
    /// # use coord::Size;
    /// # use coord::Coord;
    /// assert_eq!(Size::new(3, 4), Size::from_x_y(3, 4));
    /// ```
    fn from_x_y(x: T, y: T) -> Self {
        Self::new(x, y)
    }

    /// # Examples
    /// ```
    /// # use coord::Size;
    /// # use coord::Coord;
    /// assert_eq!(5, Size::from_x_y(5, 7).x());
    /// ```
    fn x(&self) -> T {
        self.width()
    }

    /// # Examples
    /// ```
    /// # use coord::Size;
    /// # use coord::Coord;
    /// assert_eq!(7, Size::from_x_y(5, 7).y());
    /// ```
    fn y(&self) -> T {
        self.height()
    }
}

impl<T: Value + fmt::Display> fmt::Display for Size<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.width(), self.height())
    }
}
