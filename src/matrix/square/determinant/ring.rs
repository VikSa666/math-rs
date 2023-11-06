use std::ops::{Add, Mul};

use crate::{
    identities::{One, Zero},
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
    fn zero(rows: usize, cols: usize) -> Self {
        let mut data = Vec::with_capacity(rows);
        for _ in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for _ in 0..cols {
                row.push(R::zero(0, 0));
            }
            data.push(row);
        }
        Self {
            data,
            dimension: rows,
        }
    }

    fn is_zero(&self, tolerance: f32) -> bool {
        self.data
            .iter()
            .all(|row| row.iter().all(|element| element.is_zero(tolerance)))
    }
}

impl<R: Ring> Mul for SquareMatrix<R> {
    type Output = Result<Self, super::MatrixError>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = SquareMatrix::<R>::zero(self.dimension, rhs.dimension);
        for row in 0..self.dimension {
            for column in 0..rhs.dimension {
                let mut sum = R::zero(0, 0);
                for i in 0..self.dimension {
                    sum = sum + self[(row, i)].to_owned() * rhs[(i, column)].to_owned();
                }
                result[(row, column)] = sum;
            }
        }
        Ok(result)
    }
}

impl<R: Ring> One for SquareMatrix<R> {
    fn one(rows: usize, cols: usize) -> Self {
        let mut data = Vec::with_capacity(rows);
        for row in 0..rows {
            let mut row_data = Vec::with_capacity(cols);
            for col in 0..cols {
                if row == col {
                    row_data.push(R::one(0, 0));
                } else {
                    row_data.push(R::zero(0, 0));
                }
            }
            data.push(row_data);
        }
        Self {
            data,
            dimension: rows,
        }
    }

    fn is_one(&self, tolerance: f32) -> bool {
        self.data.iter().enumerate().all(|(row, row_data)| {
            row_data
                .iter()
                .enumerate()
                .all(|(col, element)| element.is_one(tolerance) || row != col)
        })
    }
}

impl<R: Ring + PartialOrd> Group for SquareMatrix<R> {
    fn identity() -> Self {
        todo!()
    }

    fn inverse(&self) -> Self {
        todo!()
    }

    fn op(&self, rhs: &Self) -> Self {
        todo!()
    }
}
