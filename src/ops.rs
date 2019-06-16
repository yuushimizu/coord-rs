use crate::coord::Primimtive;
use crate::map::Map;
use crate::point::Point;
use crate::size::Size;
use crate::vector::Vector;
use std::ops::{Add, AddAssign, Sub, SubAssign};

impl<RHSP: Primimtive, T: Primimtive + Add<RHSP, Output = impl Primimtive>> Add<Vector<RHSP>>
    for Point<T>
{
    type Output = Point<<T as Add<RHSP>>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Point;
    /// # use coord::Vector;
    /// assert_eq!(Point::new(17, 29), Point::new(10, 20) + Vector::new(7, 9));
    /// ```
    fn add(self, rhs: Vector<RHSP>) -> Self::Output {
        (self, rhs).map(|(p, v)| p + v)
    }
}

impl<T: Primimtive, RHS> AddAssign<RHS> for Point<T>
where
    Point<T>: Add<RHS, Output = Self>,
{
    /// # Examples
    /// ```
    /// # use coord::Point;
    /// # use coord::Vector;
    /// let mut p = Point::new(10, 20);
    /// p += Vector::new(7, 9);
    /// assert_eq!(Point::new(17, 29), p);
    /// ```
    fn add_assign(&mut self, rhs: RHS) {
        *self = *self + rhs;
    }
}

impl<RHSP: Primimtive, T: Primimtive + Sub<RHSP, Output = impl Primimtive>> Sub<Vector<RHSP>>
    for Point<T>
{
    type Output = Point<<T as Sub<RHSP>>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Point;
    /// # use coord::Vector;
    /// assert_eq!(Point::new(4, 7), Point::new(14, 27) - Vector::new(10, 20));
    /// ```
    fn sub(self, rhs: Vector<RHSP>) -> Self::Output {
        (self, rhs).map(|(p, v)| p - v)
    }
}

impl<T: Primimtive, RHS> SubAssign<RHS> for Point<T>
where
    Point<T>: Sub<RHS, Output = Self>,
{
    /// # Examples
    /// ```
    /// # use coord::Point;
    /// # use coord::Vector;
    /// let mut p = Point::new(14, 27);
    /// p -= Vector::new(10, 20);
    /// assert_eq!(Point::new(4, 7), p);
    /// ```
    fn sub_assign(&mut self, rhs: RHS) {
        *self = *self - rhs;
    }
}

impl<RHSP: Primimtive, T: Primimtive + Sub<RHSP, Output = impl Primimtive>> Sub<Point<RHSP>>
    for Point<T>
{
    type Output = Vector<<T as Sub<RHSP>>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Point;
    /// # use coord::Vector;
    /// assert_eq!(Vector::new(5, 8), Point::new(15, 28) - Point::new(10, 20));
    /// ```
    fn sub(self, rhs: Point<RHSP>) -> Self::Output {
        (self, rhs).map(|(p, rhs)| p - rhs)
    }
}

impl<RHSP: Primimtive, T: Primimtive + Add<RHSP, Output = impl Primimtive>> Add<Size<RHSP>>
    for Point<T>
{
    type Output = Point<<T as Add<RHSP>>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Point;
    /// # use coord::Size;
    /// assert_eq!(Point::new(13, 24), Point::new(3, 4) + Size::new(10, 20));
    /// ```
    fn add(self, rhs: Size<RHSP>) -> Self::Output {
        (self, rhs).map(|(p, s)| p + s)
    }
}

impl<RHSP: Primimtive, T: Primimtive + Sub<RHSP, Output = impl Primimtive>> Sub<Size<RHSP>>
    for Point<T>
{
    type Output = Point<<T as Sub<RHSP>>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Point;
    /// # use coord::Size;
    /// assert_eq!(Point::new(3, 4), Point::new(13, 24) - Size::new(10, 20));
    /// ```
    fn sub(self, rhs: Size<RHSP>) -> Self::Output {
        (self, rhs).map(|(p, s)| p - s)
    }
}
