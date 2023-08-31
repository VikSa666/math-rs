use std::{
    fmt::Display,
    ops::{Add, Mul, Neg, Sub},
};

use super::{Group, Ring};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Representation of an integer number.
pub struct Integer<I>
where
    I: Add<Output = I>
        + Mul<Output = I>
        + Neg<Output = I>
        + Sub<Output = I>
        + Sized
        + Copy
        + Eq
        + Display,
{
    value: I,
}

macro_rules! impl_integer {
    ($($t:ty),*) => {
        $(impl Integer<$t> {
            pub fn new(value: $t) -> Self {
                Self { value }
            }
        })*
    };
}

impl_integer!(isize, i8, i16, i32, i64, i128);

macro_rules! impl_from_primitive_integer {
    ($($t:ty),*) => {
        $(impl From<$t> for Integer<$t> {
            fn from(value: $t) -> Self {
                Self { value }
            }
        })*
    };
}

impl_from_primitive_integer!(isize, i8, i16, i32, i64, i128);

macro_rules! impl_as_f32_and_f64 {
    ($($t:ty),*) => {
        $(impl Integer<$t> {
            pub fn as_f32(&self) -> f32 {
                self.value as f32
            }
            pub fn as_f64(&self) -> f64 {
                self.value as f64
            }
        })*
    };
}

impl_as_f32_and_f64!(isize, i8, i16, i32, i64, i128);

impl<I> Display for Integer<I>
where
    I: Add<Output = I>
        + Mul<Output = I>
        + Neg<Output = I>
        + Sub<Output = I>
        + Sized
        + Copy
        + Eq
        + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<I> Add for Integer<I>
where
    I: Add<Output = I>
        + Mul<Output = I>
        + Neg<Output = I>
        + Sub<Output = I>
        + Sized
        + Copy
        + Eq
        + Display,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl<I> Mul for Integer<I>
where
    I: Add<Output = I>
        + Mul<Output = I>
        + Neg<Output = I>
        + Sub<Output = I>
        + Sized
        + Copy
        + Eq
        + Display,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
        }
    }
}

macro_rules! impl_group_for_integer {
    ($($t:ty),*) => {
        $(impl Group for Integer<$t> {
            fn identity() -> Self {
                Self::new(0)
            }

            fn inverse(&self) -> Self {
                Self::new(-self.value)
            }

            fn op(&self, rhs: &Self) -> Self {
                self.clone() + rhs.clone()
            }
        })*
    };
}

impl_group_for_integer!(isize, i8, i16, i32, i64, i128);

macro_rules! impl_ring_for_integer {
    ($($t:ty),*) => {
        $(impl Ring for Integer<$t> {
            fn zero() -> Self {
                Self::new(0)
            }

            fn one() -> Self {
                Self::new(1)
            }

            fn is_zero(&self) -> bool {
                self == &Self::zero()
            }

            fn is_one(&self) -> bool {
                self == &Self::one()
            }

            fn sum(&self, rhs: &Self) -> Self {
                self.clone() + rhs.clone()
            }

            fn mul(&self, rhs: &Self) -> Self {
                self.clone() * rhs.clone()
            }

            fn inverse_addition(&self) -> Self {
                Self::new(-self.value)
            }
        })*
    };
}

impl_ring_for_integer!(isize, i8, i16, i32, i64, i128);

#[cfg(test)]
mod test {

    use crate::structures::{integers::Integer, Group, Ring};

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
