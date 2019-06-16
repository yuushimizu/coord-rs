use crate::coord::{Coord, Primimtive};
use crate::map::Map;
use num;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector<T: Primimtive> {
    x: T,
    y: T,
}

impl<T: Primimtive> Vector<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// # Examples
    /// ```
    /// # use coord::Vector;
    /// assert_eq!(3, Vector::new(3, 4).x());
    /// ```
    pub fn x(&self) -> T {
        self.x
    }

    /// # Examples
    /// ```
    /// # use coord::Vector;
    /// assert_eq!(4, Vector::new(3, 4).y());
    /// ```
    pub fn y(&self) -> T {
        self.y
    }

    /// # Examples
    /// ```
    /// # use coord::Vector;
    /// assert_eq!(5.0, Vector::new(3.0, 4.0).magnitude());
    /// ```
    pub fn magnitude(&self) -> T
    where
        T: num::Float,
    {
        (self.x().powi(2) + self.y().powi(2)).sqrt()
    }
}

impl<T: Primimtive> Coord for Vector<T> {
    type Item = T;

    /// # Examples
    /// ```
    /// # use coord::Vector;
    /// # use coord::Coord;
    /// assert_eq!(Vector::new(10, 20), Vector::from_x_y(10, 20));
    /// ```
    fn from_x_y(x: T, y: T) -> Self {
        Self::new(x, y)
    }

    fn x(&self) -> T {
        self.x()
    }

    fn y(&self) -> T {
        self.y()
    }
}

impl<T: Primimtive + fmt::Display> fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}

impl<T: Primimtive + Default> Default for Vector<T> {
    /// # Examples
    /// ```
    /// # use coord::Vector;
    /// assert_eq!(Vector::new(i32::default(), i32::default()), Vector::default());
    /// ```
    fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

impl<RHSP: Primimtive, T: Primimtive + Add<RHSP, Output = impl Primimtive>> Add<Vector<RHSP>> for Vector<T> {
    type Output = Vector<<T as Add<RHSP>>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Vector;
    /// assert_eq!(Vector::new(10, 15), Vector::new(3, 9) + Vector::new(7, 6));
    /// ```
    fn add(self, rhs: Vector<RHSP>) -> Self::Output {
        (self, rhs).map(|(n, m)| n + m)
    }
}

impl<T: Primimtive + Neg<Output = impl Primimtive>> Neg for Vector<T> {
    type Output = Vector<<T as Neg>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Vector;
    /// assert_eq!(Vector::new(-5, -9), -Vector::new(5, 9));
    /// ```
    fn neg(self) -> Self::Output {
        self.map(|n| -n)
    }
}

impl<RHSP: Primimtive, T: Primimtive + Sub<RHSP, Output = impl Primimtive>> Sub<Vector<RHSP>> for Vector<T> {
    type Output = Vector<<T as Sub<RHSP>>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Vector;
    /// assert_eq!(Vector::new(3, 9), Vector::new(10, 15) - Vector::new(7, 6));
    /// ```
    fn sub(self, rhs: Vector<RHSP>) -> Self::Output {
        (self, rhs).map(|(n, m)| n - m)
    }
}

impl<RHSP: Copy, T: Primimtive + Mul<RHSP, Output = impl Primimtive>> Mul<RHSP> for Vector<T> {
    type Output = Vector<<T as Mul<RHSP>>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Vector;
    /// assert_eq!(Vector::new(20, 30), Vector::new(2, 3) * 10);
    /// ```
    fn mul(self, rhs: RHSP) -> Self::Output {
        self.map(|n| n * rhs)
    }
}

impl<RHSP: Copy, T: Primimtive + Div<RHSP, Output = impl Primimtive>> Div<RHSP> for Vector<T> {
    type Output = Vector<<T as Div<RHSP>>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Vector;
    /// assert_eq!(Vector::new(6, 8), Vector::new(60, 80) / 10);
    /// ```
    fn div(self, rhs: RHSP) -> Self::Output {
        self.map(|n| n / rhs)
    }
}

impl<T: Primimtive + num::Zero> num::Zero for Vector<T> {
    /// # Examples
    /// ```
    /// # use coord::Vector;
    /// # use num::Zero;
    /// assert_eq!(Vector::new(0, 0), Vector::zero());
    /// ```
    fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }

    /// # Examples
    /// ```
    /// # use coord::Vector;
    /// # use num::Zero;
    /// assert!(Vector::new(0, 0).is_zero());
    /// ```
    fn is_zero(&self) -> bool {
        self.x().is_zero() && self.y().is_zero()
    }
}
