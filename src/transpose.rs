use crate::coord::Coord;

pub trait Transpose: Coord {
    /// # Examples
    /// ```
    /// # use coord::Vector;
    /// # use coord::Transpose;
    /// assert_eq!(Vector::new(8, 12), Vector::new(12, 8).transpose());
    /// ```
    fn transpose(&self) -> Self {
        Self::from_x_y(self.y(), self.x())
    }
}

impl<T: Coord> Transpose for T {}
