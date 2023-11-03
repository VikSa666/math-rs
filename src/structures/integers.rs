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
    num_types::{AsF32, FromF32},
    traits::Abs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
/// Representation of an integer number.
pub struct Integer<R>
where
    R: Ring + PartialOrd,
{
    value: R,
}

impl<R> Integer<R>
where
    R: Ring + PartialOrd,
{
    /// Returns a new instance of [`Integer`], given a value that can be
    /// any of the following types: [`isize`], [`i8`], [`i16`], [`i32`], [`i64`], [`i128`].
    pub fn new(value: R) -> Self {
        Self { value }
    }

    /// Returns the inside value
    pub fn value(&self) -> &R {
        &self.value
    }
}

impl<R> From<R> for Integer<R>
where
    R: Ring + PartialOrd,
{
    fn from(value: R) -> Self {
        Self { value }
    }
}

impl<R> Display for Integer<R>
where
    R: Ring + PartialOrd,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<R> Add for Integer<R>
where
    R: Ring + PartialOrd,
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
    R: Ring + PartialOrd,
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
    R: Ring + PartialOrd,
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
    R: Ring + PartialOrd,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { value: -self.value }
    }
}

impl<R> Sub for Integer<R>
where
    R: Ring + PartialOrd,
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
    R: Ring + PartialOrd,
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
    R: Ring + PartialOrd,
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
    R: Ring + PartialOrd,
{
    type Err = StructureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(R::from_str(s).map_err(|_| {
            StructureError::ParseError("Error parsing integer".to_string())
        })?))
    }
}

impl<R> Equals for Integer<R>
where
    R: Ring + PartialOrd,
{
    fn equals(&self, rhs: &Self, tolerance: f32) -> bool {
        self.value.equals(&rhs.value, tolerance)
    }
}

impl<R> AsF32 for Integer<R>
where
    R: Ring + PartialOrd + AsF32,
{
    fn as_f32(&self) -> f32 {
        self.value.as_f32()
    }
}

impl<R> FromF32 for Integer<R>
where
    R: Ring + PartialOrd + FromF32,
{
    fn from_f32(value: f32, tolerance: f32) -> Self {
        Self::new(R::from_f32(value, tolerance))
    }
}

impl<R> Group for Integer<R>
where
    R: Ring + PartialOrd,
{
    fn identity() -> Self {
        Self::new(Zero::zero())
    }

    fn inverse(&self) -> Self {
        Self::new(-self.clone().value)
    }

    fn op(&self, rhs: &Self) -> Self {
        self.clone() + rhs.clone()
    }
}

impl<R> Div for Integer<R>
where
    R: Ring + PartialOrd,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        quotient(&self, &rhs)
    }
}

impl<R> Abs for Integer<R>
where
    R: Ring + PartialOrd,
{
    type Output = Self;

    fn abs_value(&self) -> Self::Output {
        let val = self.value.abs_value();
        Self::new(val)
    }
}

impl<R> Ring for Integer<R>
where
    R: Ring + PartialOrd,
{
    fn sum(&self, rhs: &Self) -> Self {
        self.clone() + rhs.clone()
    }

    fn mul(&self, rhs: &Self) -> Self {
        self.clone() * rhs.clone()
    }
}

#[cfg(test)]
mod test {

    use std::str::FromStr;

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

    #[test]
    fn integers_from_str_should_not_fail() {
        let a = Integer::<isize>::from_str("1234");
        assert!(a.is_ok());
    }

    #[test]
    #[should_panic]
    fn integers_from_str_should_fail() {
        let a = Integer::<isize>::from_str("1234.5");
        assert!(a.is_ok());
    }
}
