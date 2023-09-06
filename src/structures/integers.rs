use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
    str::FromStr,
};

use super::{errors::StructureError, Group, Ring};
use crate::{
    arithmetics::euclid::quotient,
    equality::Equals,
    identities::{One, Zero},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Representation of an integer number.
pub struct Integer<R>
where
    R: Ring,
{
    value: R,
}

macro_rules! impl_integer {
    ($($t:ty),*) => {
        $(impl Integer<$t> {

            /// Returns the value of the [`Integer`] as an [`f32`] using Rust's built-in
            /// `as f32` conversion.
            pub fn as_f32(&self) -> f32 {
                self.value as f32
            }

            /// Returns the value of the [`Integer`] as an [`f64`] using Rust's built-in
            /// `as f64` conversion.
            pub fn as_f64(&self) -> f64 {
                self.value as f64
            }
        })*
    };
}

impl_integer!(isize, i8, i16, i32, i64, i128);

impl<R> Integer<R>
where
    R: Ring,
{
    /// Returns a new instance of [`Integer`], given a value that can be
    /// any of the following types: [`isize`], [`i8`], [`i16`], [`i32`], [`i64`], [`i128`].
    pub fn new(value: R) -> Self {
        Self { value }
    }

    /// Returns the inside value
    pub fn value(&self) -> R {
        self.value
    }
}

impl<R> From<R> for Integer<R>
where
    R: Ring,
{
    fn from(value: R) -> Self {
        Self { value }
    }
}

impl<R> Display for Integer<R>
where
    R: Ring,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<R> Add for Integer<R>
where
    R: Ring,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl<R> Mul for Integer<R>
where
    R: Ring,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
        }
    }
}

impl<R> Rem for Integer<R>
where
    R: Ring,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value % rhs.value,
        }
    }
}

impl<R> Neg for Integer<R>
where
    R: Ring,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { value: -self.value }
    }
}

impl<R> Sub for Integer<R>
where
    R: Ring,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl<R> Zero for Integer<R>
where
    R: Ring,
{
    fn zero() -> Self {
        Self::new(Zero::zero())
    }

    fn is_zero(&self, _: f32) -> bool {
        self.value.is_zero(0_f32)
    }
}

impl<R> One for Integer<R>
where
    R: Ring,
{
    fn one() -> Self {
        Self::new(One::one())
    }

    fn is_one(&self, _: f32) -> bool {
        self.value.is_one(0_f32)
    }
}

impl<R> FromStr for Integer<R>
where
    R: Ring,
{
    type Err = StructureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(R::from_str(s).map_err(|_| {
            StructureError::ParseError(format!("Error parsing integer"))
        })?))
    }
}

impl<R> Equals for Integer<R>
where
    R: Ring,
{
    fn equals(&self, rhs: &Self, tolerance: f32) -> bool {
        self.value.equals(&rhs.value, tolerance)
    }
}

impl<R> Group for Integer<R>
where
    R: Ring,
{
    fn identity() -> Self {
        Self::new(Zero::zero())
    }

    fn inverse(&self) -> Self {
        Self::new(-self.value)
    }

    fn op(&self, rhs: &Self) -> Self {
        *self + *rhs
    }
}

impl<R> Div for Integer<R>
where
    R: Ring,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        quotient(self, rhs)
    }
}

impl<R> Ring for Integer<R>
where
    R: Ring,
{
    fn sum(&self, rhs: &Self) -> Self {
        *self + *rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        *self * *rhs
    }
}

#[cfg(test)]
mod test {

    use crate::{
        identities::{One, Zero},
        structures::{integers::Integer, Group, Ring},
    };

    #[test]
    fn integers_as_group() {
        let a = Integer::<isize>::new(1);
        let b = Integer::<isize>::new(1);
        let result = b.op(&Integer::<isize>::inverse(&a));
        pretty_assertions::assert_eq!(result, Integer::<isize>::identity());
    }

    #[test]
    fn integers_as_ring() {
        let a = Integer::<i32>::new(1);
        let b = Integer::<i32>::new(1);
        let sum = a.sum(&b);
        let sub = a.sum(&Integer::<i32>::inverse_addition(&b));
        let mul = a.mul(&b);
        pretty_assertions::assert_eq!(sum, Integer::<i32>::new(2));
        pretty_assertions::assert_eq!(sub, Integer::zero());
        pretty_assertions::assert_eq!(mul, Integer::one());
    }
}
