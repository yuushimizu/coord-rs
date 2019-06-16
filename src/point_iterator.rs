use crate::axis::Axis;
use crate::axis::AxisKeyed;
use crate::coord::Primitive;
use crate::point::Point;
use crate::vector::Vector;
use std::ops;

fn next<T: Primitive, S: Primitive>(
    start: Point<T>,
    end: Point<T>,
    step: Vector<S>,
    current: &mut Point<T>,
    compare: fn(T, T) -> bool
) -> Option<Point<T>>
where
    T: PartialOrd + ops::Add<S, Output = T>,
{
    while compare(current.y(), end.y()) {
        if compare(current.x(), end.x()) {
            let result = Some(*current);
            *current = current.add(Axis::X, step.x());
            return result;
        }
        *current = Point::new(start.x(), current.y() + step.y());
    }
    None
}

pub struct PointIterator<T: Primitive, S: Primitive> {
    start: Point<T>,
    end: Point<T>,
    step: Vector<S>,
    current: Point<T>,
}

impl<T: Primitive, S: Primitive> PointIterator<T, S> {
    pub fn new(start: Point<T>, end: Point<T>, step: Vector<S>) -> Self {
        Self {
            start,
            end,
            step,
            current: start,
        }
    }
}

impl<T: Primitive, S: Primitive> Iterator for PointIterator<T, S>
where
    T: PartialOrd + ops::Add<S, Output = T>,
{
    type Item = Point<T>;

    fn next(&mut self) -> Option<Point<T>> {
        next(self.start, self.end, self.step, &mut self.current, |n, m| n < m)
    }
}

pub struct PointIteratorInclusive<T: Primitive, S: Primitive> {
    start: Point<T>,
    end: Point<T>,
    step: Vector<S>,
    current: Point<T>,
}

impl<T: Primitive, S: Primitive> PointIteratorInclusive<T, S> {
    pub fn new(start: Point<T>, end: Point<T>, step: Vector<S>) -> Self {
        Self {
            start,
            end,
            step,
            current: start,
        }
    }
}

impl<T: Primitive, S: Primitive> Iterator for PointIteratorInclusive<T, S>
where
    T: PartialOrd + ops::Add<S, Output = T>,
{
    type Item = Point<T>;

    fn next(&mut self) -> Option<Point<T>> {
        next(self.start, self.end, self.step, &mut self.current, |n, m| n <= m)
    }
}