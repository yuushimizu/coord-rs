use crate::coord::Primitive;
use crate::point::Point;
use crate::point_range_iterator::{PointRangeIterator, PointRangeIteratorInclusive, PointStep};
use crate::vector::Vector;
use std::ops;

pub trait PointRangeBounds<T: Primitive> {
    fn contains<U: Primitive>(&self, point: Point<U>) -> bool
    where
        T: PartialOrd<U>,
        U: PartialOrd<T>;
}

impl<T: Primitive, B: ops::RangeBounds<Point<T>>> PointRangeBounds<T> for B {
    /// # Examples
    /// ```
    /// # use coord::PointRangeBounds;
    /// # use coord::Point;
    /// assert!((Point::new(0, 100)..Point::new(10, 110)).contains(Point::new(3, 108)));
    /// assert!((Point::new(0, 10)..=Point::new(10, 20)).contains(Point::new(10, 20)));
    /// assert!((Point::new(10, 20)..).contains(Point::new(15, 25)));
    /// assert!((..Point::new(10, 20)).contains(Point::new(8, 17)));
    /// assert!((..=Point::new(10, 20)).contains(Point::new(10, 20)));
    /// assert!((..).contains(Point::new(100, 200)));
    /// ```
    fn contains<U: Primitive>(&self, point: Point<U>) -> bool
    where
        T: PartialOrd<U>,
        U: PartialOrd<T>,
    {
        use ops::Bound::*;
        (match self.start_bound() {
            Included(start) => start.x() <= point.x() && start.y() <= point.y(),
            Excluded(start) => start.x() < point.x() && start.y() < point.y(),
            Unbounded => true,
        }) && (match self.end_bound() {
            Included(end) => point.x() <= end.x() && point.y() <= end.y(),
            Excluded(end) => point.x() < end.x() && point.y() < end.y(),
            Unbounded => true,
        })
    }
}

pub trait PointRange<T: Primitive> {
    type PointIterator: Iterator<Item = Point<T>>;

    fn points(&self) -> Self::PointIterator;
}

impl<T: PointStep> PointRange<T> for ops::Range<Point<T>> {
    type PointIterator = PointRangeIterator<T, T>;

    /// # Examples
    /// ```
    /// # use coord::PointRange;
    /// # use coord::Point;
    /// assert_eq!(
    ///     vec![
    ///         Point::new(10, 20), Point::new(11, 20), Point::new(12, 20),
    ///         Point::new(10, 21), Point::new(11, 21), Point::new(12, 21),
    ///         Point::new(10, 22), Point::new(11, 22), Point::new(12, 22),
    ///         Point::new(10, 23), Point::new(11, 23), Point::new(12, 23)
    ///     ],
    ///     (Point::new(10, 20)..Point::new(13, 24)).points().collect::<Vec<_>>());
    /// ```
    fn points(&self) -> Self::PointIterator {
        PointRangeIterator::new(self.start, self.end, Vector::new(T::one(), T::one()))
    }
}

impl<T: PointStep> PointRange<T> for ops::RangeInclusive<Point<T>> {
    type PointIterator = PointRangeIteratorInclusive<T, T>;

    /// # Examples
    /// ```
    /// # use coord::PointRange;
    /// # use coord::Point;
    /// assert_eq!(
    ///     vec![
    ///         Point::new(10, 20), Point::new(11, 20), Point::new(12, 20), Point::new(13, 20),
    ///         Point::new(10, 21), Point::new(11, 21), Point::new(12, 21), Point::new(13, 21),
    ///         Point::new(10, 22), Point::new(11, 22), Point::new(12, 22), Point::new(13, 22),
    ///         Point::new(10, 23), Point::new(11, 23), Point::new(12, 23), Point::new(13, 23),
    ///         Point::new(10, 24), Point::new(11, 24), Point::new(12, 24), Point::new(13, 24)
    ///     ],
    ///     (Point::new(10, 20)..=Point::new(13, 24)).points().collect::<Vec<_>>());
    /// ```
    fn points(&self) -> Self::PointIterator {
        PointRangeIteratorInclusive::new(
            *self.start(),
            *self.end(),
            Vector::new(T::one(), T::one()),
        )
    }
}