use std::{fmt::Display, ops::Add};

use crate::result::{MathError, Result};

use super::{ArithmeticallyOperable, GenericMatrix, Matrix};

// impl<T> ArithmeticOperation<Result<Matrix<T>>> for Matrix<T> {}

impl<T> Add for GenericMatrix<T>
where
    T: ArithmeticallyOperable<T> + Display,
{
    type Output = Result<GenericMatrix<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.rows() != rhs.rows() || self.columns() != rhs.columns() {
            return Err(MathError::MatrixError(
                "Matrices must be of the same size".to_string(),
            ));
        } else {
            let mut result: Vec<Vec<T>> = vec![];
            for i in 0..self.rows() {
                let mut result_row: Vec<T> = vec![];
                for j in 0..self.columns() {
                    let left = self.get(i + 1, j + 1)?.to_owned();
                    let right = rhs.get(i + 1, j + 1)?.to_owned();
                    result_row.push(left + right)
                }
                result.push(result_row)
            }
            GenericMatrix::new(result)
        }
    }
}
