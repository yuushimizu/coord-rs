use num;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

pub trait AngleComponent: num::Float {
    fn pi() -> Self;

    fn double(&self) -> Self;

    fn pi_2() -> Self {
        Self::pi().double()
    }
}

impl AngleComponent for f32 {
    fn pi() -> Self {
        std::f32::consts::PI
    }

    fn double(&self) -> Self {
        self * 2.0
    }
}

impl AngleComponent for f64 {
    fn pi() -> Self {
        std::f64::consts::PI
    }

    fn double(&self) -> Self {
        self * 2.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Angle<T: AngleComponent>(T);

impl<T: AngleComponent> Angle<T> {
    pub fn new(radian: T) -> Self {
        Self(radian)
    }

    pub fn radian(&self) -> T {
        self.0
    }

    pub fn pi() -> Self {
        Self::new(T::pi())
    }

    pub fn pi_2() -> Self {
        Self::new(T::pi_2())
    }
}

impl<U: AngleComponent, T: AngleComponent + Add<U, Output = impl AngleComponent>> Add<Angle<U>>
    for Angle<T>
{
    type Output = Angle<<T as Add<U>>::Output>;

    /// # Examples
    /// ```
    /// use coord::Angle;
    /// assert_eq!(Angle::new(3.5), Angle::new(1.2) + Angle::new(2.3));
    /// ```
    fn add(self, rhs: Angle<U>) -> Self::Output {
        Self::Output::new(self.radian() + rhs.radian())
    }
}

impl<T: AngleComponent> Neg for Angle<T> {
    type Output = Angle<<T as Neg>::Output>;

    fn neg(self) -> Self::Output {
        Self::Output::new(-self.radian())
    }
}

impl<U: AngleComponent, T: AngleComponent + Sub<U, Output = impl AngleComponent>> Sub<Angle<U>>
    for Angle<T>
{
    type Output = Angle<<T as Sub<U>>::Output>;

    fn sub(self, rhs: Angle<U>) -> Self::Output {
        Self::Output::new(self.radian() - rhs.radian())
    }
}

impl<U, T: AngleComponent + Mul<U, Output = impl AngleComponent>> Mul<U> for Angle<T> {
    type Output = Angle<<T as Mul<U>>::Output>;

    fn mul(self, rhs: U) -> Self::Output {
        Self::Output::new(self.radian() * rhs)
    }
}

impl<U, T: AngleComponent + Div<U, Output = impl AngleComponent>> Div<U> for Angle<T> {
    type Output = Angle<<T as Div<U>>::Output>;

    fn div(self, rhs: U) -> Self::Output {
        Self::Output::new(self.radian() / rhs)
    }
}

impl<U, T: AngleComponent + Rem<U, Output = impl AngleComponent>> Rem<U> for Angle<T> {
    type Output = Angle<<T as Rem<U>>::Output>;

    fn rem(self, rhs: U) -> Self::Output {
        Self::Output::new(self.radian() % rhs)
    }
}
