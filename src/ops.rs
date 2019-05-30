use crate::coord::Axis;
use crate::map::Map;
use crate::point::Point;
use crate::size::Size;
use crate::vector::Vector;
use std::ops::{Add, Sub};

impl<U: Axis, T: Axis + Add<U, Output = impl Axis>> Add<Vector<U>> for Point<T> {
    type Output = Point<<T as Add<U>>::Output>;

    /// # Examples
    /// ```
    /// use coord::Point;
    /// use coord::Vector;
    /// assert_eq!(Point::new(17, 29), Point::new(10, 20) + Vector::new(7, 9));
    /// ```
    fn add(self, rhs: Vector<U>) -> Self::Output {
        (self, rhs).map(|(p, v)| p + v)
    }
}

impl<U: Axis, T: Axis + Sub<U, Output = impl Axis>> Sub<Vector<U>> for Point<T> {
    type Output = Point<<T as Sub<U>>::Output>;

    /// # Examples
    /// ```
    /// use coord::Point;
    /// use coord::Vector;
    /// assert_eq!(Point::new(4, 7), Point::new(14, 27) - Vector::new(10, 20));
    /// ```
    fn sub(self, rhs: Vector<U>) -> Self::Output {
        (self, rhs).map(|(p, v)| p - v)
    }
}

impl<U: Axis, T: Axis + Sub<U, Output = impl Axis>> Sub<Point<U>> for Point<T> {
    type Output = Vector<<T as Sub<U>>::Output>;

    /// # Examples
    /// ```
    /// use coord::Point;
    /// use coord::Vector;
    /// assert_eq!(Vector::new(5, 8), Point::new(15, 28) - Point::new(10, 20));
    /// ```
    fn sub(self, rhs: Point<U>) -> Self::Output {
        (self, rhs).map(|(p, rhs)| p - rhs)
    }
}

impl<U: Axis, T: Axis + Add<U, Output = impl Axis>> Add<Size<U>> for Point<T> {
    type Output = Point<<T as Add<U>>::Output>;

    /// # Examples
    /// ```
    /// use coord::Point;
    /// use coord::Size;
    /// assert_eq!(Point::new(13, 24), Point::new(3, 4) + Size::new(10, 20));
    /// ```
    fn add(self, rhs: Size<U>) -> Self::Output {
        (self, rhs).map(|(p, s)| p + s)
    }
}

impl<U: Axis, T: Axis + Sub<U, Output = impl Axis>> Sub<Size<U>> for Point<T> {
    type Output = Point<<T as Sub<U>>::Output>;

    /// # Examples
    /// ```
    /// use coord::Point;
    /// use coord::Size;
    /// assert_eq!(Point::new(3, 4), Point::new(13, 24) - Size::new(10, 20));
    /// ```
    fn sub(self, rhs: Size<U>) -> Self::Output {
        (self, rhs).map(|(p, s)| p - s)
    }
}
