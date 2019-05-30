use crate::coord::{Coord, Value};

pub trait Map<T: Value>: Coord<T> {
    /// # Examples
    /// ```
    /// use coord::Vector;
    /// use coord::Map;
    /// assert_eq!(Vector::new(8, 6), Vector::new(4, 3).map(|n| n * 2));
    /// ```
    fn map<U: Value, R: Coord<U>>(&self, mut f: impl FnMut(T) -> U) -> R {
        R::from_x_y(f(self.x()), f(self.y()))
    }
}

impl<T: Value, U: Coord<T>> Map<T> for U {}
