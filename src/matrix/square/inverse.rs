use crate::{matrix::error::MatrixError, structures::Ring};

use super::SquareMatrix;

impl<R: Ring> SquareMatrix<R> {
    pub fn inverse_gauss_jordan(&self, tolerance: f32) -> Result<SquareMatrix<R>, MatrixError> {
        // let reduced = self.gaussian_elimination(tolerance)?;
        todo!()
    }
}
