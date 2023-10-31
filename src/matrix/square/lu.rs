use std::ops::{Index, IndexMut};

use crate::{matrix::MatrixError, structures::Ring};

use super::SquareMatrix;

impl<R: Ring + PartialEq + PartialOrd> SquareMatrix<R> {
    /// Compute the LU decomposition of a square matrix.
    ///
    /// Source: <https://en.wikipedia.org/wiki/LU_decomposition>
    ///
    /// ## Complexity
    /// The complexity of this algorithm is O(⅔n^3).
    pub fn lu(&self) -> Result<(SquareMatrix<R>, SquareMatrix<R>), MatrixError> {
        todo!()
    }
}
