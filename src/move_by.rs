use crate::coord::Value;
use crate::point::Point;
use crate::rect::Rect;
use crate::vector::Vector;
use std::ops::Add;

pub trait MoveBy<T: Value> {
    fn move_by(self, vector: Vector<T>) -> Self;
}

impl<T: Value, U: Value> MoveBy<U> for Point<T>
where
    Point<T>: Add<Vector<U>, Output = Point<T>>,
{
    /// # Examples
    /// ```
    /// # use coord::Point;
    /// # use coord::Vector;
    /// # use coord::MoveBy;
    /// assert_eq!(Point::new(15, 17), Point::new(5, 7).move_by(Vector::new(10, 10)));
    /// ```
    fn move_by(self, vector: Vector<U>) -> Self {
        self + vector
    }
}

impl<T: Value, U: Value> MoveBy<U> for Rect<T>
where
    Point<T>: MoveBy<U>,
{
    /// # Examples
    /// ```
    /// # use coord::Rect;
    /// # use coord::Point;
    /// # use coord::Size;
    /// # use coord::Vector;
    /// # use coord::MoveBy;
    /// assert_eq!(Rect::new(Point::new(12, 24), Size::new(8, 9)), Rect::new(Point::new(10, 20), Size::new(8, 9)).move_by(Vector::new(2, 4)));
    /// ```
    fn move_by(self, vector: Vector<U>) -> Self {
        Self::new(self.origin().move_by(vector), self.size())
    }
}
