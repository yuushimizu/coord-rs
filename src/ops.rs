use crate::coord::Value;
use crate::map::Map;
use crate::point::Point;
use crate::size::Size;
use crate::vector::Vector;
use std::ops::{Add, Sub};

impl<U: Value, T: Value + Add<U, Output = impl Value>> Add<Vector<U>> for Point<T> {
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

impl<U: Value, T: Value + Sub<U, Output = impl Value>> Sub<Vector<U>> for Point<T> {
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

impl<U: Value, T: Value + Sub<U, Output = impl Value>> Sub<Point<U>> for Point<T> {
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

impl<U: Value, T: Value + Add<U, Output = impl Value>> Add<Size<U>> for Point<T> {
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

impl<U: Value, T: Value + Sub<U, Output = impl Value>> Sub<Size<U>> for Point<T> {
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
