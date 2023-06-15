use std::str::FromStr;

use crate::{result::Result, MathError};

/// # Arithmetically Operable
/// Supertrait that englobes all the traits any operable element should implement. It includes
/// - The four basic operations: addition, substraction, multiplication and division, but checked.
/// - The `Sized` trait, which is automatically implemented for any type that has a constant size known at compile-time.
/// - The `Clone` trait, which is automatically implemented for any type that implements `Copy`.
/// - The `PartialEq` trait, which is automatically implemented for any type that implements `Eq`.
/// - The `FromStr` trait, which is automatically implemented for any type that implements `FromStr`.
/// - The `Zero` trait, which will indicate how the zero element of the type is represented.
/// - The `Identity` trait, which will indicate how the identity element of the type is represented.
pub trait ArithmeticallyOperable:
    CheckedAdd + CheckedSub + CheckedMul + Sized + Clone + PartialEq + FromStr + Zero + Identity
{
}

pub trait CheckedAdd {
    type Output;
    fn checked_add(&self, rhs: &Self) -> Self::Output;
}

macro_rules! impl_checked_add {
    ($($t:ty),*) => {
        $(impl CheckedAdd for $t {
            type Output = Result<$t>;
            fn checked_add(&self, rhs: &Self) -> Self::Output {
                (*self as $t).checked_add(*rhs).ok_or(MathError::MatrixError("Addition error".to_string()))
            }
        })*
    };
}
impl_checked_add!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128);

impl CheckedAdd for f32 {
    type Output = Result<f32>;

    fn checked_add(&self, rhs: &Self) -> Self::Output {
        Ok(self + rhs)
    }
}

impl CheckedAdd for f64 {
    type Output = Result<f64>;

    fn checked_add(&self, rhs: &Self) -> Self::Output {
        Ok(self + rhs)
    }
}

pub trait CheckedSub {
    type Output;
    fn checked_sub(&self, rhs: &Self) -> Self::Output;
}

macro_rules! impl_checked_sub {
    ($($t:ty),*) => {
        $(impl CheckedSub for $t {
            type Output = Result<$t>;
            fn checked_sub(&self, rhs: &Self) -> Self::Output {
                (*self as $t).checked_sub(*rhs).ok_or(MathError::MatrixError("Substraction error".to_string()))
            }
        })*
    };
}
impl_checked_sub!(isize, i8, i16, i32, i64, i128);

impl CheckedSub for f32 {
    type Output = Result<f32>;

    fn checked_sub(&self, rhs: &Self) -> Self::Output {
        Ok(self - rhs)
    }
}

impl CheckedSub for f64 {
    type Output = Result<f64>;

    fn checked_sub(&self, rhs: &Self) -> Self::Output {
        Ok(self - rhs)
    }
}

impl CheckedSub for usize {
    type Output = Result<isize>;
    fn checked_sub(&self, rhs: &Self) -> Self::Output {
        (*self as isize)
            .checked_sub(*rhs as isize)
            .ok_or(MathError::MatrixError("Substraction error".to_string()))
    }
}

impl CheckedSub for u8 {
    type Output = Result<i8>;
    fn checked_sub(&self, rhs: &Self) -> Self::Output {
        (*self as i8)
            .checked_sub(*rhs as i8)
            .ok_or(MathError::MatrixError("Substraction error".to_string()))
    }
}

impl CheckedSub for u16 {
    type Output = Result<i16>;
    fn checked_sub(&self, rhs: &Self) -> Self::Output {
        (*self as i16)
            .checked_sub(*rhs as i16)
            .ok_or(MathError::MatrixError("Substraction error".to_string()))
    }
}

impl CheckedSub for u32 {
    type Output = Result<i32>;
    fn checked_sub(&self, rhs: &Self) -> Self::Output {
        (*self as i32)
            .checked_sub(*rhs as i32)
            .ok_or(MathError::MatrixError("Substraction error".to_string()))
    }
}

impl CheckedSub for u64 {
    type Output = Result<i64>;
    fn checked_sub(&self, rhs: &Self) -> Self::Output {
        (*self as i64)
            .checked_sub(*rhs as i64)
            .ok_or(MathError::MatrixError("Substraction error".to_string()))
    }
}

impl CheckedSub for u128 {
    type Output = Result<i128>;
    fn checked_sub(&self, rhs: &Self) -> Self::Output {
        (*self as i128)
            .checked_sub(*rhs as i128)
            .ok_or(MathError::MatrixError("Substraction error".to_string()))
    }
}

pub trait CheckedDiv {
    type Output;
    fn checked_div(&self, rhs: &Self) -> Self::Output;
}

macro_rules! impl_checked_div {
    ($($t:ty),*) => {
        $(impl CheckedDiv for $t {
            type Output = Result<$t>;
            fn checked_div(&self, rhs: &Self) -> Self::Output {
                (*self as $t).checked_div(*rhs).ok_or(MathError::MatrixError("Division error".to_string()))
            }
        })*
    };
}

impl_checked_div!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128);

impl CheckedDiv for f32 {
    type Output = Result<f32>;

    fn checked_div(&self, rhs: &Self) -> Self::Output {
        if rhs == &0.0 {
            Err(MathError::MathError("Division by zero".to_string()))
        } else {
            Ok(self / rhs)
        }
    }
}

impl CheckedDiv for f64 {
    type Output = Result<f64>;

    fn checked_div(&self, rhs: &Self) -> Self::Output {
        Ok(self / rhs)
    }
}

pub trait CheckedMul {
    type Output;
    fn checked_mul(&self, rhs: &Self) -> Self::Output;
}

macro_rules! impl_checked_mul {
    ($($t:ty),*) => {
        $(impl CheckedMul for $t {
            type Output = Result<$t>;
            fn checked_mul(&self, rhs: &Self) -> Self::Output {
                (*self as $t).checked_mul(*rhs).ok_or(MathError::MatrixError("Multiplication error".to_string()))
            }
        })*
    };
}

impl_checked_mul!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128);

impl CheckedMul for f32 {
    type Output = Result<f32>;

    fn checked_mul(&self, rhs: &Self) -> Self::Output {
        Ok(self * rhs)
    }
}

impl CheckedMul for f64 {
    type Output = Result<f64>;

    fn checked_mul(&self, rhs: &Self) -> Self::Output {
        Ok(self * rhs)
    }
}

pub trait Zero {
    fn zero(rows: usize, columns: usize, tolerance: f32) -> Self;
    fn is_zero(&self) -> bool;
}

macro_rules! impl_zero {
    ($($t:ty),*) => {
        $(impl Zero for $t {
            fn zero(_rows: usize, _columns: usize, _tolerance: f32) -> Self {
                0 as $t
            }
            fn is_zero(&self) -> bool {
                *self == 0 as $t
            }
        })*
    };
}

impl_zero!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);

pub trait Identity {
    fn id(dimensions: usize, tolerance: f32) -> Self;
}

macro_rules! impl_identity {
    ($($t:ty),*) => {
        $(impl Identity for $t {
            fn id(_dimensions: usize, _tolerance: f32) -> Self {
                1 as $t
            }
        })*
    };
}

impl_identity!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);

macro_rules! impl_arithmetically_opreable {
    ($($t:ty),*) => {
        $(impl ArithmeticallyOperable for $t {})*
    }
}

impl_arithmetically_opreable!(
    usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64
);

pub trait Parseable {
    fn parse(string: &str, tolerance: f32) -> Result<Self>
    where
        Self: Sized;
}

pub trait Serializable {
    /// Serialize the matrix, return it in the form `{{a, b, c}, {d, e, f}, {g, h, i}}`
    fn serialize(&self) -> String;
}
