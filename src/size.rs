use crate::coord::{Axis, Coord};
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size<T: Axis> {
    width: T,
    height: T,
}

impl<T: Axis> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    /// # Examples
    /// ```
    /// use coord::Size;
    /// assert_eq!(10, Size::new(10, 20).width());
    /// ```
    pub fn width(&self) -> T {
        self.width
    }

    /// # Examples
    /// ```
    /// use coord::Size;
    /// assert_eq!(20, Size::new(10, 20).height());
    /// ```
    pub fn height(&self) -> T {
        self.height
    }
}

impl<T: Axis> Coord<T> for Size<T> {
    /// # Examples
    /// ```
    /// use coord::Size;
    /// use coord::Coord;
    /// assert_eq!(Size::new(3, 4), Size::from_x_y(3, 4));
    /// ```
    fn from_x_y(x: T, y: T) -> Self {
        Self::new(x, y)
    }

    /// # Examples
    /// ```
    /// use coord::Size;
    /// use coord::Coord;
    /// assert_eq!(5, Size::from_x_y(5, 7).x());
    /// ```
    fn x(&self) -> T {
        self.width()
    }

    /// # Examples
    /// ```
    /// use coord::Size;
    /// use coord::Coord;
    /// assert_eq!(7, Size::from_x_y(5, 7).y());
    /// ```
    fn y(&self) -> T {
        self.height()
    }
}

impl<T: Axis + Mul<T>> Size<T> {
    /// # Examples
    /// ```
    /// use coord::Size;
    /// assert_eq!(12, Size::new(3, 4).area());
    /// ```
    pub fn area(&self) -> <T as Mul<T>>::Output {
        self.width() * self.height()
    }
}
