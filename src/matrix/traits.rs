use std::ops::{Add, Div, Mul, Sub};

use crate::result::Result;

/// Supertrait representing all the elementary arithmetic operations
pub trait ArithmeticOperation<T>:
    Add<Output = T> + Sub<Output = T> + Div<Output = T> + Mul<Output = T> + Sized + Clone + Copy
{
}

impl<T> ArithmeticOperation<T> for T where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + Mul<Output = T> + Sized + Clone + Copy
{
}

pub trait Matrix<T: ArithmeticOperation<T>> {
    fn columns(&self) -> usize;
    fn rows(&self) -> usize;
    fn is_square(&self) -> bool;
    fn get(&self, row: usize, column: usize) -> Result<&T>;
    fn set(&mut self, row: usize, column: usize, value: T) -> Result<()>;
}
