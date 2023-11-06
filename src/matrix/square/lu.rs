use crate::{
    matrix::{AsMatrix, MatrixError},
    structures::Ring,
};

use super::SquareMatrix;

impl<R: Ring + PartialEq + PartialOrd> SquareMatrix<R> {
    /// Compute the LU decomposition of a square matrix.
    ///
    /// Source: <https://en.wikipedia.org/wiki/LU_decomposition>
    ///
    /// ## Complexity
    /// The complexity of this algorithm is O(⅔n^3).
    pub fn lu(&self) -> Result<(SquareMatrix<R>, SquareMatrix<R>), MatrixError> {
        let n = self.rows();
        let mut l = SquareMatrix::identity(n);
        let mut u = self.clone();

        for k in 0..n {
            for i in k + 1..n {
                l[(i, k)] = u[(i, k)] / u[(k, k)];
                for j in k..n {
                    u[(i, j)] = u[(i, j)] - l[(i, k)] * u[(k, j)];
                }
            }
        }

        Ok((l, u))
    }
}
