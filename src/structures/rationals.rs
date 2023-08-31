use std::{
    fmt::Display,
    ops::{Add, Mul, Neg, Sub},
};

use super::integers::Integer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rational<I>
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
    numerator: Integer<I>,
    denominator: Integer<I>,
}

macro_rules! impl_rational {
    ($($t:ty),*) => {
        $(impl Rational<$t> {
            pub fn new(numerator: $t, denominator: $t) -> Self {
                Self {
                    numerator: Integer::<$t>::new(numerator),
                    denominator: Integer::<$t>::new(denominator),
                }
            }

            pub fn as_f32(&self) -> f32 {
                self.numerator.as_f32() / self.denominator.as_f32()
            }
        })*
    };
}

impl_rational!(isize, i8, i16, i32, i64, i128);

macro_rules! impl_from_f32_for_rational {
    ($($t:ty),*) => {
        $(impl From<$t> for Rational<$t> {
            fn from(value: $t) -> Self {
                todo!()
            }
        })*
    };
}
