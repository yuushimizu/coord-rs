use crate::coord::{Coord, Value};
use crate::point::Point;
use crate::size::Size;
use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect<T: Value> {
    origin: Point<T>,
    size: Size<T>,
}

impl<T: Value> Rect<T> {
    pub fn new(origin: Point<T>, size: Size<T>) -> Self {
        Self { origin, size }
    }

    /// # Examples
    /// ```
    /// # use coord::Rect;
    /// # use coord::Point;
    /// # use coord::Size;
    /// assert_eq!(Point::new(4, 5), Rect::new(Point::new(4, 5), Size::new(10, 20)).origin());
    /// ```
    pub fn origin(&self) -> Point<T> {
        self.origin
    }

    /// # Examples
    /// ```
    /// # use coord::Rect;
    /// # use coord::Point;
    /// # use coord::Size;
    /// assert_eq!(Size::new(10, 20), Rect::new(Point::new(4, 5), Size::new(10, 20)).size());
    /// ```
    pub fn size(&self) -> Size<T> {
        self.size
    }

    /// # Examples
    /// ```
    /// # use coord::Rect;
    /// # use coord::Point;
    /// # use coord::Size;
    /// assert_eq!(Point::new(13, 24), Rect::new(Point::new(3, 4), Size::new(10, 20)).max());
    /// ```
    pub fn max(&self) -> Point<T>
    where
        Point<T>: Add<Size<T>, Output = Point<T>>,
    {
        self.origin() + self.size()
    }

    /// # Examples
    /// ```
    /// # use coord::Rect;
    /// # use coord::Point;
    /// # use coord::Size;
    /// assert!(Rect::new(Point::new(5, 10), Size::new(4, 8)).contains(Point::new(7, 13)))
    /// ```
    pub fn contains(&self, point: Point<T>) -> bool
    where
        T: PartialOrd,
        Point<T>: Add<Size<T>, Output = Point<T>>,
    {
        self.origin.x() <= point.x()
            && self.origin.y() <= point.y()
            && point.x() < self.max().x()
            && point.y() < self.max().y()
    }
}

impl<T: Value> Coord for Rect<T> {
    type Item = (T, T);

    /// # Examples
    /// ```
    /// # use coord::Rect;
    /// # use coord::Point;
    /// # use coord::Size;
    /// # use coord::Coord;
    /// assert_eq!(Rect::new(Point::new(3, 4), Size::new(10, 20)), Rect::from_x_y((3, 10), (4, 20)));
    /// ```
    fn from_x_y((x_origin, x_size): (T, T), (y_origin, y_size): (T, T)) -> Self {
        Self::new(Point::new(x_origin, y_origin), Size::new(x_size, y_size))
    }

    /// # Examples
    /// ```
    /// # use coord::Rect;
    /// # use coord::Point;
    /// # use coord::Size;
    /// # use coord::Coord;
    /// assert_eq!((1, 5), Rect::new(Point::new(1, 2), Size::new(5, 6)).x());
    /// ```
    fn x(&self) -> (T, T) {
        (self.origin().x(), self.size().x())
    }

    /// # Examples
    /// ```
    /// # use coord::Rect;
    /// # use coord::Point;
    /// # use coord::Size;
    /// # use coord::Coord;
    /// assert_eq!((2, 6), Rect::new(Point::new(1, 2), Size::new(5, 6)).y());
    /// ```
    fn y(&self) -> (T, T) {
        (self.origin().y(), self.size().y())
    }
}

impl<T: Value + fmt::Display> fmt::Display for Rect<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.origin(), self.size())
    }
}
