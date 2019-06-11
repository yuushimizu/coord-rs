use crate::coord::Coord;

pub trait Map: Coord {
    /// # Examples
    /// ```
    /// # use coord::Vector;
    /// # use coord::Map;
    /// assert_eq!(Vector::new(8, 6), Vector::new(4, 3).map(|n| n * 2));
    /// ```
    fn map<R: Coord>(&self, mut f: impl FnMut(Self::Item) -> <R as Coord>::Item) -> R {
        R::from_x_y(f(self.x()), f(self.y()))
    }
}

impl<T: Coord> Map for T {}
