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

impl<V: Value, T: MoveBy<V>> MoveBy<V> for (T,) {
    /// # Examples
    /// ```
    /// # use coord::Point;
    /// # use coord::Vector;
    /// # use coord::MoveBy;
    /// assert_eq!((Point::new(15, 27),), (Point::new(5, 7),).move_by(Vector::new(10, 20)));
    /// ```
    fn move_by(self, vector: Vector<V>) -> Self {
        (self.0.move_by(vector),)
    }
}

impl<V: Value, T0: MoveBy<V>, T1: MoveBy<V>> MoveBy<V> for (T0, T1) {
    /// # Examples
    /// ```
    /// # use coord::Point;
    /// # use coord::Size;
    /// # use coord::Rect;
    /// # use coord::Vector;
    /// # use coord::MoveBy;
    /// assert_eq!(
    ///     (Point::new(11, 22), Rect::new(Point::new(13, 24), Size::new(5, 6))),
    ///     (Point::new(1, 2), Rect::new(Point::new(3, 4), Size::new(5, 6))).move_by(Vector::new(10, 20)));
    /// ```
    fn move_by(self, vector: Vector<V>) -> Self {
        (self.0.move_by(vector), self.1.move_by(vector))
    }
}

impl<V: Value, T0: MoveBy<V>, T1: MoveBy<V>, T2: MoveBy<V>> MoveBy<V> for (T0, T1, T2) {
    /// # Examples
    /// ```
    /// # use coord::Point;
    /// # use coord::Vector;
    /// # use coord::MoveBy;
    /// assert_eq!(
    ///     (Point::new(11, 22), Point::new(13, 24), Point::new(15, 26)),
    ///     (Point::new(1, 2), Point::new(3, 4), Point::new(5, 6)).move_by(Vector::new(10, 20)));
    /// ```
    fn move_by(self, vector: Vector<V>) -> Self {
        (
            self.0.move_by(vector),
            self.1.move_by(vector),
            self.2.move_by(vector),
        )
    }
}
