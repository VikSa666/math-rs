use std::{
    ops::{Add, Div, Mul, Neg, Rem, Sub},
    str::FromStr,
};

use crate::{
    equality::Equals,
    identities::{One, Zero},
};

use super::{errors::StructureError, Field, Group, Ring};

#[derive(Clone, Copy, Debug)]
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

impl PartialEq for Real {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
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
        let x = Real::new(1.23456789);
        let y = Real::new(1.23411111);
        [1e-1, 1e-2, 1e-3].iter().for_each(|tolerance| {
            assert!(x.equals(&y, *tolerance));
        });
        [1e-4, 1e-8, 1e-12]
            .iter()
            .for_each(|tolerance| assert!(!x.equals(&y, *tolerance)))
    }
}
