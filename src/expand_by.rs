use crate::coord::Primitive;
use crate::map::Map;
use crate::rect::Rect;
use crate::size::Size;
use crate::vector::Vector;
use std::ops::Add;

pub trait ExpandBy<T: Primitive> {
    fn expand_by(self, vector: Vector<T>) -> Self;
}

impl<VP: Primitive, T: Primitive + Add<VP, Output = T>> ExpandBy<VP> for Size<T> {
    /// # Examples
    /// ```
    /// # use coord::Size;
    /// # use coord::Vector;
    /// # use coord::ExpandBy;
    /// assert_eq!(Size::new(140, 180), Size::new(100, 100).expand_by(Vector::new(40, 80)));
    /// ```
    fn expand_by(self, vector: Vector<VP>) -> Self {
        (self, vector).map(|(s, v)| s + v)
    }
}

impl<T: Primitive, VP: Primitive> ExpandBy<VP> for Rect<T>
where
    Size<T>: ExpandBy<VP>,
{
    /// # Examples
    /// ```
    /// # use coord::Rect;
    /// # use coord::Point;
    /// # use coord::Size;
    /// # use coord::Vector;
    /// # use coord::ExpandBy;
    /// assert_eq!(Rect::new(Point::new(2, 3), Size::new(14, 15)), Rect::new(Point::new(2, 3), Size::new(4, 5)).expand_by(Vector::new(10, 10)));
    /// ```
    fn expand_by(self, vector: Vector<VP>) -> Self {
        Self::new(self.origin(), self.size().expand_by(vector))
    }
}
