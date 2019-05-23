use crate::coord::{Axis, Coord};
use crate::point::Point;
use crate::size::Size;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect<T: Axis> {
    origin: Point<T>,
    size: Size<T>,
}

impl<T: Axis> Rect<T> {
    pub fn new(origin: Point<T>, size: Size<T>) -> Self {
        Self { origin, size }
    }

    /// # Example
    /// ```
    /// use coord::Rect;
    /// use coord::Point;
    /// use coord::Size;
    /// assert_eq!(Point::new(4, 5), Rect::new(Point::new(4, 5), Size::new(10, 20)).origin());
    /// ```
    pub fn origin(&self) -> Point<T> {
        self.origin
    }

    /// # Example
    /// ```
    /// use coord::Rect;
    /// use coord::Point;
    /// use coord::Size;
    /// assert_eq!(Size::new(10, 20), Rect::new(Point::new(4, 5), Size::new(10, 20)).size());
    /// ```
    pub fn size(&self) -> Size<T> {
        self.size
    }
}

pub type RectAxis<T> = (T, T);

impl<T: Axis> Coord<(T, T)> for Rect<T> {
    /// # Example
    /// ```
    /// use coord::Rect;
    /// use coord::Point;
    /// use coord::Size;
    /// use coord::Coord;
    /// assert_eq!(Rect::new(Point::new(3, 4), Size::new(10, 20)), Rect::from_x_y((3, 10), (4, 20)));
    /// ```
    fn from_x_y((x_origin, x_size): RectAxis<T>, (y_origin, y_size): RectAxis<T>) -> Self {
        Self::new(Point::new(x_origin, y_origin), Size::new(x_size, y_size))
    }

    /// # Example
    /// ```
    /// use coord::Rect;
    /// use coord::Point;
    /// use coord::Size;
    /// use coord::Coord;
    /// assert_eq!((1, 5), Rect::new(Point::new(1, 2), Size::new(5, 6)).x());
    /// ```
    fn x(&self) -> RectAxis<T> {
        (self.origin().x(), self.size().x())
    }

    /// # Example
    /// ```
    /// use coord::Rect;
    /// use coord::Point;
    /// use coord::Size;
    /// use coord::Coord;
    /// assert_eq!((2, 6), Rect::new(Point::new(1, 2), Size::new(5, 6)).y());
    /// ```
    fn y(&self) -> RectAxis<T> {
        (self.origin().y(), self.size().y())
    }
}
