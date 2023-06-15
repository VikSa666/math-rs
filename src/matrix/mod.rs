mod float;
pub mod macros;

pub use float::MatrixF32;
pub use macros::*;

use crate::result::Result;
use crate::traits::{ArithmeticallyOperable, Parseable, Serializable};

pub trait Invertible {
    fn inverse_gauss_jordan(&self) -> Result<Self>
    where
        Self: Sized;

    fn inverse_montante(&self) -> Result<Self>
    where
        Self: Sized;

    fn inverse_adjoint(&self) -> Result<Self>
    where
        Self: Sized;
}

pub trait Matrix: ArithmeticallyOperable + Invertible + Parseable + Serializable {
    type T: ArithmeticallyOperable;
    /// Will return the number of columns of the matrix
    fn columns(&self) -> usize;

    /// Will return the number of rows of the matrix
    fn rows(&self) -> usize;

    /// Will return `true` if the matrix is squared, i.e., if `rows == columns`
    fn is_square(&self) -> bool;

    /// Will return `true` if the matrix is symmetric, i.e., if `A == A^T`
    fn is_symmetric(&self) -> bool;

    /// Get a reference of an element of the matrix, or error if you provide wrong indexes
    fn get(&self, row: usize, column: usize) -> Result<&Self::T>;

    /// Get a mutable reference of an element of the matrix, or error if you provide wrong indexes
    fn get_mut(&mut self, row: usize, column: usize) -> Result<&mut Self::T>;

    /// Set an element of the matrix, or error if you provide wrong indexes
    fn set(&mut self, row: usize, column: usize, value: Self::T) -> Result<()>;

    /// Swap two rows of the matrix, or error if you provide wrong indexes
    fn swap_rows(&mut self, row1: usize, row2: usize) -> Result<()>;

    /// Return a new matrix being the transposed of the current one. It does not eliminate the current one
    fn transpose(&self) -> Self;

    /// Return a new matrix being the reduced gaussian inferior triangular of the current one. It does not eliminate the current one
    fn gaussian_triangulation(&self) -> Result<Self>;

    /// Returns a tuple containing the matrices `L` and `U` of the LU decomposition, in order. It does not eliminate the current one
    fn lu_decomposition(&self) -> Result<(Self, Self)>;

    /// Returns a matrix resulting from the Cholesky decomposition. It does not eliminate the current one
    fn cholesky_decomposition(&self) -> Result<Self>;

    /// Return the determinant or a `None` if the matrix is not squared
    fn determinant_using_gauss(&self) -> Option<Self::T>;

    fn determinant_using_lu(&self) -> Option<Self::T>;
}
