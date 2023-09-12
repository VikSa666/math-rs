use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
    str::FromStr,
};

use super::{errors::StructureError, integers::Integer, Field, Group, Ring};

use crate::{
    arithmetics::euclid,
    equality::Equals,
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

            pub fn approx_from_f32(number: f32, tolerance: f32) -> Self {
                let int_part = number as $t;
                let decimal = number - (int_part as f32);

                let int_part_fraction = Rational::<$t>::new(Integer::new(int_part), Integer::one());
                let decimal_fraction = Rational::<$t>::new(
                    Integer::<$t>::new((decimal * (1. / tolerance)) as $t),
                    Integer::<$t>::new((1. / tolerance) as $t),
                );

                int_part_fraction + decimal_fraction
            }
        })*
    };
}

impl_rationals_as_f32!(isize, i8, i16, i32, i64, i128);

macro_rules! impl_rational_from_primitives {
    ($($t:ty),*) => {
        $(impl From<$t> for Rational<$t> {
            fn from(value: $t) -> Self {
                Self::new(Integer::new(value), Integer::one())
            }
        })*
    };
}

impl_rational_from_primitives!(isize, i8, i16, i32, i64, i128);

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

    fn is_zero(&self, _: f32) -> bool {
        self.equals(&Self::zero(), 0.)
    }
}

impl<R> One for Rational<R>
where
    R: Ring,
{
    fn one() -> Self {
        Self::new(Integer::one(), Integer::one())
    }

    fn is_one(&self, _: f32) -> bool {
        self.equals(&Self::one(), 0.)
    }
}

impl<R> FromStr for Rational<R>
where
    R: Ring,
{
    type Err = StructureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('/');
        let numerator = split
            .next()
            .and_then(|num| num.parse::<Integer<R>>().ok())
            .ok_or(StructureError::ParseError("Invalid numerator".to_string()))?;
        let denominator = split
            .next()
            .and_then(|denom| denom.parse::<Integer<R>>().ok())
            .ok_or(StructureError::ParseError(
                "Invalid denominator".to_string(),
            ))?;
        Ok(Self::new(numerator, denominator))
    }
}

impl<R> Equals for Rational<R>
where
    R: Ring,
{
    fn equals(&self, rhs: &Self, tolerance: f32) -> bool {
        (self.numerator * rhs.denominator).equals(&(self.denominator * rhs.numerator), tolerance)
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
}

impl<R> Field for Rational<R>
where
    R: Ring,
{
    fn inverse_multiplication(&self) -> Self {
        Self {
            numerator: self.denominator.clone(),
            denominator: self.numerator.clone(),
        }
        .simplified()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_rational_should_not_fail() {
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

    #[test]
    fn build_rational_from_f32() {
        struct Test<'a> {
            name: &'a str,
            input: f32,
            epsilon: f32,
            expected: Rational<i128>,
        }

        vec![
            Test {
                name: "easy integer",
                input: 1.0,
                epsilon: 1e-4,
                expected: Rational::<i128>::new(Integer::<i128>::new(1), Integer::<i128>::new(1)),
            },
            Test {
                name: "easy one decimal",
                input: 1.5,
                epsilon: 1e-4,
                expected: Rational::<i128>::new(Integer::<i128>::new(3), Integer::<i128>::new(2)),
            },
            Test {
                name: "easy two decimals",
                input: 1.25,
                epsilon: 1e-4,
                expected: Rational::<i128>::new(Integer::<i128>::new(5), Integer::<i128>::new(4)),
            },
            Test {
                name: "medium random decimals",
                input: 1.23456789,
                epsilon: 1e-4,
                expected: Rational::<i128>::new(
                    Integer::<i128>::new(2469),
                    Integer::<i128>::new(2000),
                ),
            },
            Test {
                name: "medium random decimals",
                input: 1.23456789,
                epsilon: 1e-12,
                expected: Rational::<i128>::new(
                    Integer::<i128>::new(5796311),
                    Integer::<i128>::new(4695012),
                ),
            },
        ]
        .into_iter()
        .for_each(|test| {
            pretty_assertions::assert_eq!(
                Rational::<i128>::approx_from_f32(test.input, test.epsilon),
                test.expected,
                "Test {} with epsilon = {} (computed vs expected)",
                test.name,
                test.epsilon
            )
        });
    }
}
