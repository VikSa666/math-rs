use std::{
    ops::{Add, Div, Mul, Neg, Rem, Sub},
    str::FromStr,
};

use crate::{
    equality::Equals,
    identities::{One, Zero},
    num_types::{AsF32, FromF32},
    traits::Abs,
};

use super::{errors::StructureError, Field, Group, Ring};

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub struct Real {
    value: f32,
}

impl Real {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn sqrt(&self) -> Self {
        Self::new(self.value.sqrt())
    }

    pub fn abs(&self) -> Self {
        Self::new(self.value.abs())
    }
}

impl Add for Real {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl Zero for Real {
    fn zero() -> Self {
        Self::new(0_f32)
    }

    fn is_zero(&self, tolerance: f32) -> bool {
        self.equals(&Self::zero(), tolerance)
    }
}

impl FromStr for Real {
    type Err = StructureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(f32::from_str(s)?))
    }
}

impl std::fmt::Display for Real {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:+}", self.value)
    }
}

impl Neg for Real {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.value)
    }
}

impl Sub for Real {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.value - rhs.value)
    }
}

impl Equals for Real {
    fn equals(&self, rhs: &Self, tolerance: f32) -> bool {
        (self.value - rhs.value).abs() < tolerance
    }
}

impl FromF32 for Real {
    fn from_f32(value: f32, _: f32) -> Self {
        Self::new(value)
    }
}

impl AsF32 for Real {
    fn as_f32(&self) -> f32 {
        self.value
    }
}

impl Abs for Real {
    type Output = Real;
    fn abs_value(&self) -> Self::Output {
        Self::new(self.value.abs())
    }
}

impl Group for Real {
    fn identity() -> Self {
        Self::zero()
    }

    fn inverse(&self) -> Self {
        Self::new(-self.value)
    }

    fn op(&self, rhs: &Self) -> Self {
        *self + *rhs
    }
}

impl Mul for Real {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.value * rhs.value)
    }
}

impl One for Real {
    fn one() -> Self {
        Self::new(1_f32)
    }

    fn is_one(&self, tolerance: f32) -> bool {
        self.value.equals(&1_f32, tolerance)
    }
}

impl Rem for Real {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::new(self.value % rhs.value)
    }
}

impl Div for Real {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.value / rhs.value)
    }
}

impl Ring for Real {
    fn sum(&self, rhs: &Self) -> Self {
        *self + *rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        *self * *rhs
    }
}

impl Field for Real {
    fn inverse_multiplication(&self) -> Self {
        Self::new(1_f32 / self.value)
    }
}

#[cfg(test)]
mod test {
    use crate::equality::Equals;

    use super::Real;

    #[test]
    fn equals_with_tolerance() {
        let x = Real::new(1.234_567_9);
        let y = Real::new(1.234_111_1);
        [1e-1, 1e-2, 1e-3].iter().for_each(|tolerance| {
            assert!(x.equals(&y, *tolerance));
        });
        [1e-4, 1e-8, 1e-12]
            .iter()
            .for_each(|tolerance| assert!(!x.equals(&y, *tolerance)))
    }

    #[test]
    fn operations() {
        let x = Real::new(1.234_567_9);
        let y = Real::new(1.234_111_1);
        let sum = x + y;
        let sub = x - y;
        let mul = x * y;
        let div = x / y;
        [1e-4, 1e-8, 1e-12]
            .into_iter()
            .for_each(|tolerance| assert!(sum.equals(&Real::new(2.468679), tolerance)));
        [1e-4, 1e-8, 1e-12]
            .into_iter()
            .for_each(|tolerance| assert!(sub.equals(&Real::new(0.00045681), tolerance)));
        [1e-4, 1e-8, 1e-12]
            .into_iter()
            .for_each(|tolerance| assert!(mul.equals(&Real::new(1.5235939), tolerance)));
        [1e-4, 1e-8, 1e-12]
            .into_iter()
            .for_each(|tolerance| assert!(div.equals(&Real::new(1.0003701), tolerance)));
    }
}
