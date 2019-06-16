use num;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

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

impl<T: AnglePrimitive + Default> Default for Angle<T> {
    /// # Examples
    /// ```
    /// # use coord::Angle;
    /// assert_eq!(Angle::new(f32::default()), Angle::default());
    /// ```
    fn default() -> Self {
        Self::new(T::default())
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

impl<T: AnglePrimitive, RHS> AddAssign<RHS> for Angle<T>
where
    Angle<T>: Add<RHS, Output = Self>,
{
    /// # Examples
    /// ```
    /// # use coord::Angle;
    /// let mut a = Angle::new(1.2);
    /// a += Angle::new(2.3);
    /// assert_eq!(Angle::new(3.5), a);
    /// ```
    fn add_assign(&mut self, rhs: RHS) {
        *self = *self + rhs;
    }
}

impl<T: AnglePrimitive> Neg for Angle<T> {
    type Output = Angle<<T as Neg>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Angle;
    /// assert_eq!(Angle::new(-1.0), -Angle::new(1.0));
    /// ```
    fn neg(self) -> Self::Output {
        Self::Output::new(-self.radian())
    }
}

impl<RHSP: AnglePrimitive, T: AnglePrimitive + Sub<RHSP, Output = impl AnglePrimitive>>
    Sub<Angle<RHSP>> for Angle<T>
{
    type Output = Angle<<T as Sub<RHSP>>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Angle;
    /// assert_eq!(Angle::new(2.3), Angle::new(3.5) - Angle::new(1.2));
    /// ```
    fn sub(self, rhs: Angle<RHSP>) -> Self::Output {
        Self::Output::new(self.radian() - rhs.radian())
    }
}

impl<T: AnglePrimitive, RHS> SubAssign<RHS> for Angle<T>
where
    Angle<T>: Sub<RHS, Output = Self>,
{
    /// # Examples
    /// ```
    /// # use coord::Angle;
    /// let mut a = Angle::new(3.5);
    /// a -= Angle::new(1.2);
    /// assert_eq!(Angle::new(2.3), a);
    /// ```
    fn sub_assign(&mut self, rhs: RHS) {
        *self = *self - rhs;
    }
}

impl<RHSP, T: AnglePrimitive + Mul<RHSP, Output = impl AnglePrimitive>> Mul<RHSP> for Angle<T> {
    type Output = Angle<<T as Mul<RHSP>>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Angle;
    /// assert_eq!(Angle::new(6.0), Angle::new(3.0) * 2.0);
    /// ```
    fn mul(self, rhs: RHSP) -> Self::Output {
        Self::Output::new(self.radian() * rhs)
    }
}

impl<T: AnglePrimitive, RHS> MulAssign<RHS> for Angle<T>
where
    Angle<T>: Mul<RHS, Output = Self>,
{
    /// # Examples
    /// ```
    /// # use coord::Angle;
    /// let mut a = Angle::new(3.0);
    /// a *= 2.0;
    /// assert_eq!(Angle::new(6.0), a);
    /// ```
    fn mul_assign(&mut self, rhs: RHS) {
        *self = *self * rhs;
    }
}

impl<RHSP, T: AnglePrimitive + Div<RHSP, Output = impl AnglePrimitive>> Div<RHSP> for Angle<T> {
    type Output = Angle<<T as Div<RHSP>>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Angle;
    /// assert_eq!(Angle::new(3.0), Angle::new(6.0) / 2.0);
    /// ```
    fn div(self, rhs: RHSP) -> Self::Output {
        Self::Output::new(self.radian() / rhs)
    }
}

impl<T: AnglePrimitive, RHS> DivAssign<RHS> for Angle<T>
where
    Angle<T>: Div<RHS, Output = Self>,
{
    /// # Examples
    /// ```
    /// # use coord::Angle;
    /// let mut a = Angle::new(6.0);
    /// a /= 2.0;
    /// assert_eq!(Angle::new(3.0), a);
    /// ```
    fn div_assign(&mut self, rhs: RHS) {
        *self = *self / rhs;
    }
}

impl<RHSP, T: AnglePrimitive + Rem<RHSP, Output = impl AnglePrimitive>> Rem<RHSP> for Angle<T> {
    type Output = Angle<<T as Rem<RHSP>>::Output>;

    /// # Examples
    /// ```
    /// # use coord::Angle;
    /// assert_eq!(Angle::new(1.0), Angle::new(5.0) % 2.0);
    /// ```
    fn rem(self, rhs: RHSP) -> Self::Output {
        Self::Output::new(self.radian() % rhs)
    }
}

impl<T: AnglePrimitive, RHS> RemAssign<RHS> for Angle<T>
where
    Angle<T>: Rem<RHS, Output = Self>,
{
    /// # Examples
    /// ```
    /// # use coord::Angle;
    /// let mut a = Angle::new(5.0);
    /// a %= 2.0;
    /// assert_eq!(Angle::new(1.0), a);
    /// ```
    fn rem_assign(&mut self, rhs: RHS) {
        *self = *self % rhs;
    }
}
