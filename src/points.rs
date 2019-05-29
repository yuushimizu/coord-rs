use crate::coord::Axis;
use crate::point::Point;
use crate::rect::Rect;
use crate::size::Size;
use crate::vector::Vector;
use num::{range_step, range_step_inclusive, CheckedAdd, One, ToPrimitive, Zero};

pub trait StepAxis: Clone + Zero + One + ToPrimitive + PartialOrd + CheckedAdd {}

impl<T: Clone + Zero + One + ToPrimitive + PartialOrd + CheckedAdd> StepAxis for T {}

impl<T: Axis + StepAxis> Size<T> {
    fn points_step_by<R: Iterator<Item = T>>(
        &self,
        origin: Point<T>,
        step: Vector<T>,
        range_step: impl Fn(T, T, T) -> R,
    ) -> impl Iterator<Item = Point<T>> {
        let stop_x = origin.x() + self.width();
        range_step(origin.y(), origin.y() + self.height(), step.y()).flat_map(move |y| {
            range_step(origin.x(), stop_x, step.x()).map(move |x| Point::new(x, y))
        })
    }

    /// # Examples
    /// ```
    /// use coord::Size;
    /// use coord::Point;
    /// use coord::Vector;
    /// assert_eq!(
    ///     vec![
    ///         Point::new(3, 2), Point::new(5, 2), Point::new(7, 2),
    ///         Point::new(3, 3), Point::new(5, 3), Point::new(7, 3)
    ///     ],
    ///     Size::new(6, 2).points_step(Point::new(3, 2), Vector::new(2, 1)).collect::<Vec<_>>());
    /// ```
    pub fn points_step(&self, origin: Point<T>, step: Vector<T>) -> impl Iterator<Item = Point<T>> {
        self.points_step_by(origin, step, range_step)
    }

    /// # Examples
    /// ```
    /// use coord::Size;
    /// use coord::Point;
    /// assert_eq!(
    ///     vec![
    ///         Point::new(3, 2), Point::new(4, 2), Point::new(5, 2),
    ///         Point::new(3, 3), Point::new(4, 3), Point::new(5, 3)
    ///     ],
    ///     Size::new(3, 2).points(Point::new(3, 2)).collect::<Vec<_>>());
    /// ```
    pub fn points(&self, origin: Point<T>) -> impl Iterator<Item = Point<T>> {
        self.points_step(origin, Vector::new(T::one(), T::one()))
    }

    /// # Examples
    /// ```
    /// use coord::Size;
    /// use coord::Point;
    /// use coord::Vector;
    /// assert_eq!(
    ///     vec![
    ///         Point::new(3, 2), Point::new(5, 2), Point::new(7, 2), Point::new(9, 2),
    ///         Point::new(3, 3), Point::new(5, 3), Point::new(7, 3), Point::new(9, 3),
    ///         Point::new(3, 4), Point::new(5, 4), Point::new(7, 4), Point::new(9, 4)
    ///     ],
    ///     Size::new(6, 2).points_step_inclusive(Point::new(3, 2), Vector::new(2, 1)).collect::<Vec<_>>());
    /// ```
    pub fn points_step_inclusive(
        &self,
        origin: Point<T>,
        step: Vector<T>,
    ) -> impl Iterator<Item = Point<T>> {
        self.points_step_by(origin, step, range_step_inclusive)
    }

    /// # Examples
    /// ```
    /// use coord::Size;
    /// use coord::Point;
    /// assert_eq!(
    ///     vec![
    ///         Point::new(3, 2), Point::new(4, 2), Point::new(5, 2), Point::new(6, 2),
    ///         Point::new(3, 3), Point::new(4, 3), Point::new(5, 3), Point::new(6, 3),
    ///         Point::new(3, 4), Point::new(4, 4), Point::new(5, 4), Point::new(6, 4)
    ///     ],
    ///     Size::new(3, 2).points_inclusive(Point::new(3, 2)).collect::<Vec<_>>());
    /// ```
    pub fn points_inclusive(&self, origin: Point<T>) -> impl Iterator<Item = Point<T>> {
        self.points_step_inclusive(origin, Vector::new(T::one(), T::one()))
    }
}

impl<T: Axis + StepAxis> Rect<T> {
    /// # Examples
    /// ```
    /// use coord::Rect;
    /// use coord::Point;
    /// use coord::Size;
    /// assert_eq!(
    ///     vec![
    ///         Point::new(10, 20), Point::new(11, 20),
    ///         Point::new(10, 21), Point::new(11, 21),
    ///         Point::new(10, 22), Point::new(11, 22)
    ///     ],
    ///     Rect::new(Point::new(10, 20), Size::new(2, 3)).points().collect::<Vec<_>>());
    /// ```
    pub fn points(&self) -> impl Iterator<Item = Point<T>> {
        self.size().points(self.origin())
    }
}
