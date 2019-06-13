use crate::coord::Value;
use crate::map::Map;
use crate::point::Point;
use crate::size::Size;
use crate::vector::Vector;
use std::ops::{Add, Sub};

impl<RHSV: Value, T: Value + Add<RHSV, Output = impl Value>> Add<Vector<RHSV>> for Point<T> {
    type Output = Point<<T as Add<RHSV>>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Point;
    /// # use coord::Vector;
    /// assert_eq!(Point::new(17, 29), Point::new(10, 20) + Vector::new(7, 9));
    /// ```
    fn add(self, rhs: Vector<RHSV>) -> Self::Output {
        (self, rhs).map(|(p, v)| p + v)
    }
}

impl<RHSV: Value, T: Value + Sub<RHSV, Output = impl Value>> Sub<Vector<RHSV>> for Point<T> {
    type Output = Point<<T as Sub<RHSV>>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Point;
    /// # use coord::Vector;
    /// assert_eq!(Point::new(4, 7), Point::new(14, 27) - Vector::new(10, 20));
    /// ```
    fn sub(self, rhs: Vector<RHSV>) -> Self::Output {
        (self, rhs).map(|(p, v)| p - v)
    }
}

impl<RHSV: Value, T: Value + Sub<RHSV, Output = impl Value>> Sub<Point<RHSV>> for Point<T> {
    type Output = Vector<<T as Sub<RHSV>>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Point;
    /// # use coord::Vector;
    /// assert_eq!(Vector::new(5, 8), Point::new(15, 28) - Point::new(10, 20));
    /// ```
    fn sub(self, rhs: Point<RHSV>) -> Self::Output {
        (self, rhs).map(|(p, rhs)| p - rhs)
    }
}

impl<RHSV: Value, T: Value + Add<RHSV, Output = impl Value>> Add<Size<RHSV>> for Point<T> {
    type Output = Point<<T as Add<RHSV>>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Point;
    /// # use coord::Size;
    /// assert_eq!(Point::new(13, 24), Point::new(3, 4) + Size::new(10, 20));
    /// ```
    fn add(self, rhs: Size<RHSV>) -> Self::Output {
        (self, rhs).map(|(p, s)| p + s)
    }
}

impl<RHSV: Value, T: Value + Sub<RHSV, Output = impl Value>> Sub<Size<RHSV>> for Point<T> {
    type Output = Point<<T as Sub<RHSV>>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Point;
    /// # use coord::Size;
    /// assert_eq!(Point::new(3, 4), Point::new(13, 24) - Size::new(10, 20));
    /// ```
    fn sub(self, rhs: Size<RHSV>) -> Self::Output {
        (self, rhs).map(|(p, s)| p - s)
    }
}
