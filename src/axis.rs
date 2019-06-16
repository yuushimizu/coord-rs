use crate::coord::Coord;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    /// # Examples
    /// ```
    /// # use coord::Axis::{X, Y};
    /// assert_eq!(Y, X.transpose());
    /// assert_eq!(X, Y.transpose());
    /// ```
    pub fn transpose(self) -> Axis {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::X,
        }
    }
}

pub trait AxisKeyed: Coord {
    /// # Examples
    /// ```
    /// # use coord::Vector;
    /// # use coord::Axis;
    /// # use coord::AxisKeyed;
    /// let v = Vector::new(30, 50);
    /// assert_eq!(30, v.get(Axis::X));
    /// assert_eq!(50, v.get(Axis::Y));
    /// ```
    fn get(&self, axis: Axis) -> Self::Item {
        match axis {
            Axis::X => self.x(),
            Axis::Y => self.y(),
        }
    }

    /// # Examples
    /// ```
    /// # use coord::Point;
    /// # use coord::Axis;
    /// # use coord::AxisKeyed;
    /// let p = Point::new(5, 8);
    /// assert_eq!(Point::new(10, 8), Point::new(5, 8).set(Axis::X, 10));
    /// assert_eq!(Point::new(5, 10), Point::new(5, 8).set(Axis::Y, 10));
    /// ```
    fn set(&self, axis: Axis, value: Self::Item) -> Self {
        match axis {
            Axis::X => Self::from_x_y(value, self.y()),
            Axis::Y => Self::from_x_y(self.x(), value),
        }
    }

    /// # Examples
    /// ```
    /// # use coord::Point;
    /// # use coord::Axis;
    /// # use coord::AxisKeyed;
    /// let p = Point::new(10, 20);
    /// assert_eq!(Point::new(12, 20), Point::new(10, 20).add(Axis::X, 2));
    /// assert_eq!(Point::new(10, 22), Point::new(10, 20).add(Axis::Y, 2));
    /// ```
    fn add<T>(&self, axis: Axis, amount: T) -> Self
    where
        Self::Item: ops::Add<T, Output = Self::Item>,
    {
        self.set(axis, self.get(axis) + amount)
    }
}

impl<T: Coord> AxisKeyed for T {}
