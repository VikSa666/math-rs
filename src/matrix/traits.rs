use std::{
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

use crate::result::Result;

/// Supertrait representing all the elementary arithmetic operations
pub trait ArithmeticallyOperable<T>:
    Add<Output = T>
    + Sub<Output = T>
    + Div<Output = T>
    + Mul<Output = T>
    + Sized
    + Clone
    + Copy
    + PartialEq
    + FromStr
{
}

impl<T> ArithmeticallyOperable<T> for T where
    T: Add<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sized
        + Clone
        + Copy
        + PartialEq
        + FromStr
{
}

pub trait Matrix<T: ArithmeticallyOperable<T>> {
    fn columns(&self) -> usize;
    fn rows(&self) -> usize;
    fn is_square(&self) -> bool;
    fn get(&self, row: usize, column: usize) -> Result<&T>;
    fn get_mut(&mut self, row: usize, column: usize) -> Result<&mut T>;
    fn set(&mut self, row: usize, column: usize, value: T) -> Result<()>;
}

pub trait Parseable<T: ArithmeticallyOperable<T>> {
    type Mat: Matrix<T>;
    fn parse(expr: &str) -> Result<Self::Mat>;
}
