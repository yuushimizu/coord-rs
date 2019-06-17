use crate::axis::Axis;
use crate::axis::AxisKeyed;
use crate::coord::Primitive;
use crate::point::Point;
use crate::vector::Vector;
use std::ops;

pub trait PointStep<S: Primitive = Self>:
    Primitive + PartialOrd + ops::Add<S, Output = Self> + num::One
{
}

impl<S: Primitive, T: Primitive + PartialOrd + ops::Add<S, Output = Self> + num::One> PointStep<S>
    for T
{
}

fn next<S: Primitive, T: PointStep<S>>(
    start: Point<T>,
    end: Point<T>,
    step: Vector<S>,
    current: &mut Point<T>,
    compare: fn(T, T) -> bool,
) -> Option<Point<T>> {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl<S: Primitive, T: PointStep<S>> Iterator for PointIterator<T, S> {
    type Item = Point<T>;

    fn next(&mut self) -> Option<Point<T>> {
        next(
            self.start,
            self.end,
            self.step,
            &mut self.current,
            |n, m| n < m,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl<S: Primitive, T: PointStep<S>> Iterator for PointIteratorInclusive<T, S> {
    type Item = Point<T>;

    fn next(&mut self) -> Option<Point<T>> {
        next(
            self.start,
            self.end,
            self.step,
            &mut self.current,
            |n, m| n <= m,
        )
    }
}