use num;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

pub trait AnglePrimitive: num::Float {
    fn pi() -> Self;

    fn double(&self) -> Self;

    fn pi_2() -> Self {
        Self::pi().double()
    }
}

impl AnglePrimitive for f32 {
    fn pi() -> Self {
        std::f32::consts::PI
    }

    fn double(&self) -> Self {
        self * 2.0
    }
}

impl AnglePrimitive for f64 {
    fn pi() -> Self {
        std::f64::consts::PI
    }

    fn double(&self) -> Self {
        self * 2.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Angle<T: AnglePrimitive>(T);

impl<T: AnglePrimitive> Angle<T> {
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

impl<RHSP: AnglePrimitive, T: AnglePrimitive + Add<RHSP, Output = impl AnglePrimitive>>
    Add<Angle<RHSP>> for Angle<T>
{
    type Output = Angle<<T as Add<RHSP>>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Angle;
    /// assert_eq!(Angle::new(3.5), Angle::new(1.2) + Angle::new(2.3));
    /// ```
    fn add(self, rhs: Angle<RHSP>) -> Self::Output {
        Self::Output::new(self.radian() + rhs.radian())
    }
}

impl<T: AnglePrimitive> Neg for Angle<T> {
    type Output = Angle<<T as Neg>::Output>;

    fn neg(self) -> Self::Output {
        Self::Output::new(-self.radian())
    }
}

impl<RHSP: AnglePrimitive, T: AnglePrimitive + Sub<RHSP, Output = impl AnglePrimitive>>
    Sub<Angle<RHSP>> for Angle<T>
{
    type Output = Angle<<T as Sub<RHSP>>::Output>;

    fn sub(self, rhs: Angle<RHSP>) -> Self::Output {
        Self::Output::new(self.radian() - rhs.radian())
    }
}

impl<RHSP, T: AnglePrimitive + Mul<RHSP, Output = impl AnglePrimitive>> Mul<RHSP> for Angle<T> {
    type Output = Angle<<T as Mul<RHSP>>::Output>;

    fn mul(self, rhs: RHSP) -> Self::Output {
        Self::Output::new(self.radian() * rhs)
    }
}

impl<RHSP, T: AnglePrimitive + Div<RHSP, Output = impl AnglePrimitive>> Div<RHSP> for Angle<T> {
    type Output = Angle<<T as Div<RHSP>>::Output>;

    fn div(self, rhs: RHSP) -> Self::Output {
        Self::Output::new(self.radian() / rhs)
    }
}

impl<RHSP, T: AnglePrimitive + Rem<RHSP, Output = impl AnglePrimitive>> Rem<RHSP> for Angle<T> {
    type Output = Angle<<T as Rem<RHSP>>::Output>;

    fn rem(self, rhs: RHSP) -> Self::Output {
        Self::Output::new(self.radian() % rhs)
    }
}
