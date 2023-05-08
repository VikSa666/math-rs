use std::{fmt::Display, ops::Add};

use crate::result::{MathError, Result};

use super::{
    traits::{ArithmeticOperation, Matrix},
    GenericMatrix,
};

// impl<T> ArithmeticOperation<Result<Matrix<T>>> for Matrix<T> {}

impl<T> Add for GenericMatrix<T>
where
    T: ArithmeticOperation<T> + Display,
{
    type Output = Result<GenericMatrix<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows || self.columns != rhs.columns {
            return Err(MathError::MatrixError(
                "Matrices must be of the same size".to_string(),
            ));
        } else {
            let mut result: Vec<T> = vec![];
            for i in 0..self.rows {
                for j in 0..self.columns {
                    let left = self.get(i + 1, j + 1)?.to_owned();
                    let right = rhs.get(i + 1, j + 1)?.to_owned();
                    result.push(left + right)
                }
            }
            GenericMatrix::new(result, self.rows, self.columns)
        }
    }
}
