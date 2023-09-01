use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
    str::FromStr,
};

use super::{integers::Integer, Group, Ring};

use crate::{
    arithmetics::euclid,
    identities::{One, Zero},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rational<R>
where
    R: Ring,
{
    numerator: Integer<R>,
    denominator: Integer<R>,
}

macro_rules! impl_rationals_as_f32 {
    ($($t:ty),*) => {
        $(impl Rational<$t> {
            /// Returns the result of the division of [`self.numerator`] and [`self.denominator`]
            pub fn as_f32(&self) -> f32 {
                self.numerator.as_f32() / self.denominator.as_f32()
            }
        })*
    };
}

impl_rationals_as_f32!(isize, i8, i16, i32, i64, i128);

impl<R> Rational<R>
where
    R: Ring,
{
    pub fn new(numerator: Integer<R>, denominator: Integer<R>) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub fn simplified(mut self) -> Self {
        let gcd = euclid::gcd(self.numerator, self.denominator);
        self.numerator = Integer::<R>::new(self.numerator.value() / gcd.value());
        self.denominator = Integer::<R>::new(self.denominator.value() / gcd.value());
        self
    }
}

impl<R> Display for Rational<R>
where
    R: Ring,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl<R> Add for Rational<R>
where
    R: Ring,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            self.denominator * rhs.denominator,
        )
        .simplified()
    }
}

impl<R> Mul for Rational<R>
where
    R: Ring,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
        .simplified()
    }
}

impl<R> Rem for Rational<R>
where
    R: Ring,
{
    type Output = Self;

    fn rem(self, _: Self) -> Self::Output {
        Self {
            numerator: Integer::zero(),
            denominator: Integer::zero(),
        }
        .simplified()
    }
}

impl<R> Neg for Rational<R>
where
    R: Ring,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
        .simplified()
    }
}

impl<R> Sub for Rational<R>
where
    R: Ring,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<R> Zero for Rational<R>
where
    R: Ring,
{
    fn zero() -> Self {
        Self::new(Integer::zero(), Integer::one())
    }

    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}

impl<R> One for Rational<R>
where
    R: Ring,
{
    fn one() -> Self {
        Self::new(Integer::one(), Integer::one())
    }

    fn is_one(&self) -> bool {
        self == &Self::one()
    }
}

impl<R> FromStr for Rational<R>
where
    R: Ring,
{
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('/');
        let numerator = split.next().unwrap().parse::<Integer<R>>().unwrap();
        let denominator = split.next().unwrap().parse::<Integer<R>>().unwrap();
        Ok(Self::new(numerator, denominator))
    }
}

impl<R> Group for Rational<R>
where
    R: Ring,
{
    fn identity() -> Self {
        Self::new(Integer::zero(), Integer::one())
    }

    fn inverse(&self) -> Self {
        Self::new(self.numerator.inverse(), self.denominator)
    }

    fn op(&self, rhs: &Self) -> Self {
        *self + *rhs
    }
}

impl<R> Div for Rational<R>
where
    R: Ring,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator * rhs.denominator,
            denominator: self.denominator * rhs.numerator,
        }
        .simplified()
    }
}

impl<R> Ring for Rational<R>
where
    R: Ring,
{
    fn sum(&self, rhs: &Self) -> Self {
        self.clone() + rhs.clone()
    }

    fn mul(&self, rhs: &Self) -> Self {
        self.clone() * rhs.clone()
    }

    fn inverse_addition(&self) -> Self {
        self.inverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rational() {
        let a = Rational::<isize>::new(Integer::<isize>::new(1), Integer::<isize>::new(2));
        let b = Rational::<isize>::new(Integer::<isize>::new(1), Integer::<isize>::new(3));
        let c = Rational::<isize>::new(Integer::<isize>::new(1), Integer::<isize>::new(6));
        pretty_assertions::assert_eq!(
            a + b,
            Rational::<isize>::new(Integer::<isize>::new(5), Integer::<isize>::new(6))
        );
        pretty_assertions::assert_eq!(
            a * b,
            Rational::<isize>::new(Integer::<isize>::new(1), Integer::<isize>::new(6))
        );
        pretty_assertions::assert_eq!(
            a + b + c,
            Rational::<isize>::new(Integer::<isize>::new(1), Integer::<isize>::new(1))
        );
        pretty_assertions::assert_eq!(
            a * b * c,
            Rational::<isize>::new(Integer::<isize>::new(1), Integer::<isize>::new(36))
        );
        pretty_assertions::assert_eq!(
            a + b + c + c,
            Rational::<isize>::new(Integer::<isize>::new(7), Integer::<isize>::new(6))
        );
        pretty_assertions::assert_eq!(
            a * b * c * c,
            Rational::<isize>::new(Integer::<isize>::new(1), Integer::<isize>::new(216))
        );
        pretty_assertions::assert_eq!(
            a + b + c + c + c,
            Rational::<isize>::new(Integer::<isize>::new(4), Integer::<isize>::new(3))
        );
        pretty_assertions::assert_eq!(
            a * b * c * c * c,
            Rational::<isize>::new(Integer::<isize>::new(1), Integer::<isize>::new(1296))
        );
    }
}
