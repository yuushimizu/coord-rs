use crate::coord::{Axis, Coord};

pub trait Map<T: Axis>: Coord<T> {
    /// Example
    /// ```
    /// use coord::Vector;
    /// use coord::Map;
    /// assert_eq!(Vector::new(8, 6), Vector::new(4, 3).map(|n| n * 2));
    /// ```
    fn map<U: Axis, R: Coord<U>, F: FnMut(T) -> U>(&self, mut f: F) -> R {
        R::from_x_y(f(self.x()), f(self.y()))
    }
}

impl<T: Axis, U: Coord<T>> Map<T> for U {}
