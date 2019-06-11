use crate::coord::Coord;

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
    /// let vector = Vector::new(30, 50);
    /// assert_eq!(30, vector.get(Axis::X));
    /// assert_eq!(50, vector.get(Axis::Y));
    /// ```
    fn get(&self, axis: Axis) -> Self::Item {
        match axis {
            Axis::X => self.x(),
            Axis::Y => self.y(),
        }
    }
}

impl<T: Coord> AxisKeyed for T {}
