use std::ops::Add;

use crate::{
    identities::Zero,
    matrix::{square::SquareMatrix, MatrixError},
    structures::{Group, Ring},
};

impl<R: Ring> Add for SquareMatrix<R> {
    type Output = Result<Self, MatrixError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.dimension != rhs.dimension {
            return Err(super::MatrixError::InvalidNumberOfRows);
        }
        let mut result = self.clone();
        for (row, row_elements) in self.data.iter().enumerate() {
            for (column, element) in row_elements.iter().enumerate() {
                let rhs_element = &rhs[(row, column)];
                result[(row, column)] = element.clone() + rhs_element.clone();
            }
        }
        Ok(result)
    }
}

impl<R: Ring> Zero for SquareMatrix<R> {
    fn zero() -> Self {
        todo!()
    }

    fn is_zero(&self, tolerance: f32) -> bool {
        todo!()
    }
}

// impl<R: Ring> Group for SquareMatrix<R> {
//     fn inverse(&self) -> Self {
//         todo!()
//     }
// }
