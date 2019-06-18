use crate::coord::Primitive;
use crate::point::Point;
use crate::rect::Rect;
use crate::size::Size;
use crate::vector::Vector;
use crate::point_range_iterator::{PointRangeIterator, PointRangeIteratorInclusive, PointStep};
use crate::coord_range::CoordRange;
use std::ops;

impl<T: Primitive> Size<T> where Point<T>: ops::Add<Self, Output = Point<T>> {
    /// # Examples
    /// ```
    /// # use coord::Size;
    /// # use coord::Point;
    /// # use coord::Vector;
    /// assert_eq!(
    ///     vec![
    ///         Point::new(3, 2), Point::new(5, 2), Point::new(7, 2),
    ///         Point::new(3, 3), Point::new(5, 3), Point::new(7, 3)
    ///     ],
    ///     Size::new(6, 2).points_step(Point::new(3, 2), Vector::new(2, 1)).collect::<Vec<_>>());
    /// ```
    pub fn points_step<S: Primitive>(&self, origin: Point<T>, step: Vector<S>) -> impl Iterator<Item = Point<T>> where T: PointStep<S> {
        PointRangeIterator::new(origin, origin + *self, step)
    }

    /// # Examples
    /// ```
    /// # use coord::Size;
    /// # use coord::Point;
    /// assert_eq!(
    ///     vec![
    ///         Point::new(3, 2), Point::new(4, 2), Point::new(5, 2),
    ///         Point::new(3, 3), Point::new(4, 3), Point::new(5, 3)
    ///     ],
    ///     Size::new(3, 2).points(Point::new(3, 2)).collect::<Vec<_>>());
    /// ```
    pub fn points(&self, origin: Point<T>) -> impl Iterator<Item = Point<T>> where T: PointStep {
        (origin..(origin + *self)).points()
    }

    /// # Examples
    /// ```
    /// # use coord::Size;
    /// # use coord::Point;
    /// # use coord::Vector;
    /// assert_eq!(
    ///     vec![
    ///         Point::new(3, 2), Point::new(5, 2), Point::new(7, 2), Point::new(9, 2),
    ///         Point::new(3, 3), Point::new(5, 3), Point::new(7, 3), Point::new(9, 3),
    ///         Point::new(3, 4), Point::new(5, 4), Point::new(7, 4), Point::new(9, 4)
    ///     ],
    ///     Size::new(6, 2).points_step_inclusive(Point::new(3, 2), Vector::new(2, 1)).collect::<Vec<_>>());
    /// ```
    pub fn points_step_inclusive<S: Primitive>(
        &self,
        origin: Point<T>,
        step: Vector<S>,
    ) -> impl Iterator<Item = Point<T>> where T: PointStep<S> {
        PointRangeIteratorInclusive::new(origin, origin + *self, step)
    }

    /// # Examples
    /// ```
    /// # use coord::Size;
    /// # use coord::Point;
    /// assert_eq!(
    ///     vec![
    ///         Point::new(3, 2), Point::new(4, 2), Point::new(5, 2), Point::new(6, 2),
    ///         Point::new(3, 3), Point::new(4, 3), Point::new(5, 3), Point::new(6, 3),
    ///         Point::new(3, 4), Point::new(4, 4), Point::new(5, 4), Point::new(6, 4)
    ///     ],
    ///     Size::new(3, 2).points_inclusive(Point::new(3, 2)).collect::<Vec<_>>());
    /// ```
    pub fn points_inclusive(&self, origin: Point<T>) -> impl Iterator<Item = Point<T>> where T: PointStep {
        (origin..=(origin + *self)).points()
    }
}

impl<T: Primitive> Rect<T> {
    /// # Examples
    /// ```
    /// # use coord::Rect;
    /// # use coord::Point;
    /// # use coord::Size;
    /// assert_eq!(
    ///     vec![
    ///         Point::new(10, 20), Point::new(11, 20),
    ///         Point::new(10, 21), Point::new(11, 21),
    ///         Point::new(10, 22), Point::new(11, 22)
    ///     ],
    ///     Rect::new(Point::new(10, 20), Size::new(2, 3)).points().collect::<Vec<_>>());
    /// ```
    pub fn points(&self) -> impl Iterator<Item = Point<T>> where T: PointStep {
        self.size().points(self.origin())
    }
}
