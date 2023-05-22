use std::str::FromStr;

use num::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Zero};

use crate::result::Result;

/// Supertrait representing all the elementary arithmetic operations
pub trait ArithmeticallyOperable<T>:
    CheckedAdd + CheckedSub + CheckedDiv + CheckedMul + Sized + Clone + PartialEq + FromStr + Zero
{
}

impl<T> ArithmeticallyOperable<T> for T where
    T: CheckedAdd
        + CheckedSub
        + CheckedDiv
        + CheckedMul
        + Sized
        + Clone
        + PartialEq
        + FromStr
        + Zero
{
}

pub trait Matrix<T: ArithmeticallyOperable<T>> {
    fn columns(&self) -> usize;
    fn rows(&self) -> usize;

    fn is_square(&self) -> bool;
    fn is_symmetric(&self) -> bool;

    fn get(&self, row: usize, column: usize) -> Result<&T>;
    fn get_mut(&mut self, row: usize, column: usize) -> Result<&mut T>;
    fn set(&mut self, row: usize, column: usize, value: T) -> Result<()>;
    fn swap_rows(&mut self, row1: usize, row2: usize) -> Result<()>;

    fn transpose(&self) -> Self;
    fn determinant(&self) -> Option<T>;
}
