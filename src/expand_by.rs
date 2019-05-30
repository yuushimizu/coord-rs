use crate::coord::Value;
use crate::map::Map;
use crate::rect::Rect;
use crate::size::Size;
use crate::vector::Vector;
use std::ops::Add;

pub trait ExpandBy<T: Value> {
    fn expand_by(self, vector: Vector<T>) -> Self;
}

impl<U: Value, T: Value + Add<U, Output = T>> ExpandBy<U> for Size<T> {
    /// # Examples
    /// ```
    /// use coord::Size;
    /// use coord::Vector;
    /// use coord::ExpandBy;
    /// assert_eq!(Size::new(140, 180), Size::new(100, 100).expand_by(Vector::new(40, 80)));
    /// ```
    fn expand_by(self, vector: Vector<U>) -> Self {
        (self, vector).map(|(s, v)| s + v)
    }
}

impl<T: Value, U: Value> ExpandBy<U> for Rect<T>
where
    Size<T>: ExpandBy<U>,
{
    /// # Examples
    /// ```
    /// use coord::Rect;
    /// use coord::Point;
    /// use coord::Size;
    /// use coord::Vector;
    /// use coord::ExpandBy;
    /// assert_eq!(Rect::new(Point::new(2, 3), Size::new(14, 15)), Rect::new(Point::new(2, 3), Size::new(4, 5)).expand_by(Vector::new(10, 10)));
    /// ```
    fn expand_by(self, vector: Vector<U>) -> Self {
        Self::new(self.origin(), self.size().expand_by(vector))
    }
}
