use crate::{structures::Ring, matrix::error::MatrixError};

use super::{dimension::Dimension, SquareMatrix};

impl<R: Ring> SquareMatrix<R> {
    pub fn determinant(&self) -> Result<R, MatrixError> {
        if self.dimension() == 1 {
            return self.get(0, 0)?.clone();
        }
        if self.dimension() == 3 {}
        todo!()
    }
}
