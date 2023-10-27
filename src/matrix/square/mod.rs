pub mod determinant;
pub mod equality;
pub mod parser;

use crate::structures::Ring;

use super::{error::MatrixError, AsMatrix};

#[derive(Debug, Clone, PartialEq)]
pub struct SquareMatrix<R>
where
    R: Ring,
{
    dimension: usize,
    data: Vec<Vec<R>>,
}

impl<R> SquareMatrix<R>
where
    R: Ring,
{
    pub fn new(dimension: usize, data: Vec<Vec<R>>) -> Self {
        Self { dimension, data }
    }

    /// Creates a new [`SquareMatrix`] with the given `dimension` and a given function
    /// or closure that takes the indexes as arguments.
    ///
    /// ## Example
    ///
    /// Given the parameters
    /// * `dimension = 3`,
    /// * `fn f(i: usize, j: usize) { i + j }`
    /// we would get the matrix
    /// ```txt
    /// 0 1 2
    /// 1 2 3
    /// 2 3 4
    /// ```
    pub fn from_fn(dimension: usize, f: fn(i: usize, j: usize) -> R) -> Self {
        let mut data = vec![vec![R::zero(); dimension]; dimension];
        for i in 0..dimension {
            for j in 0..dimension {
                data[i][j] = f(i, j)
            }
        }
        Self::new(dimension, data)
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    pub fn diagonal_is_zero(&self, tolerance: f32) -> bool {
        for row in 0..self.dimension() {
            if self.data[row][row].is_zero(tolerance) {
                return true;
            }
        }
        false
    }
}

impl<R: Ring> TryFrom<Vec<Vec<R>>> for SquareMatrix<R> {
    type Error = MatrixError;

    fn try_from(value: Vec<Vec<R>>) -> Result<Self, Self::Error> {
        let dimension = value.len();
        for row in value.iter() {
            if row.len() != dimension {
                return Err(MatrixError::NonSquareMatrix);
            }
        }
        Ok(Self {
            dimension,
            data: value,
        })
    }
}

impl<R: Ring> Default for SquareMatrix<R> {
    fn default() -> Self {
        Self {
            dimension: 0,
            data: Default::default(),
        }
    }
}

impl<R: Ring> std::fmt::Display for SquareMatrix<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for row in self.data.iter() {
            output.push_str(&format!("{:?}\n", row));
        }
        write!(f, "{}", output)
    }
}

impl<R> AsMatrix<R> for SquareMatrix<R>
where
    R: Ring + PartialOrd,
{
    fn data(&self) -> &Vec<Vec<R>> {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Vec<Vec<R>> {
        &mut self.data
    }

    fn with_capacity(rows: usize, _: usize) -> Self {
        let data: Vec<Vec<R>> = Vec::with_capacity(rows);
        Self {
            dimension: rows,
            data,
        }
    }

    fn rows(&self) -> usize {
        self.dimension
    }

    fn columns(&self) -> usize {
        self.dimension
    }

    fn row_iter(&self) -> std::slice::Iter<'_, Vec<R>> {
        self.data.iter()
    }

    fn get(&self, row: usize, column: usize) -> Result<&R, super::error::MatrixError> {
        self.data
            .get(row)
            .ok_or(super::error::MatrixError::RowOutOfBounds(row))?
            .get(column)
            .ok_or(super::error::MatrixError::ColumnOutOfBounds(column))
    }

    fn get_mut(&mut self, row: usize, column: usize) -> Result<&mut R, super::error::MatrixError> {
        self.data_mut()
            .get_mut(row)
            .ok_or(super::error::MatrixError::RowOutOfBounds(row))?
            .get_mut(column)
            .ok_or(super::error::MatrixError::ColumnOutOfBounds(column))
    }

    fn set(
        &mut self,
        row: usize,
        column: usize,
        value: R,
    ) -> Result<(), super::error::MatrixError> {
        let element = self.get_mut(row, column)?;
        *element = value;
        Ok(())
    }

    fn transpose(&self) -> Self {
        let mut elements = Vec::with_capacity(self.columns());
        for column in 0..self.columns() {
            let mut new_row = Vec::with_capacity(self.rows());
            for row in 0..self.rows() {
                new_row.push(self.data[row][column].clone());
            }
            elements.push(new_row);
        }
        Self {
            dimension: self.dimension,
            data: elements,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::square::SquareMatrix;
    use crate::matrix::AsMatrix;
    use crate::structures::integers::Integer;
    use crate::structures::rationals::Rational;

    #[test]
    fn test_square_matrix() {
        let matrix = SquareMatrix::new(
            2,
            vec![
                vec![Integer::new(1), Integer::new(2)],
                vec![Integer::new(3), Integer::new(4)],
            ],
        );
        assert_eq!(matrix.dimension(), 2);
        assert_eq!(
            matrix.data(),
            &vec![
                vec![Integer::new(1), Integer::new(2)],
                vec![Integer::new(3), Integer::new(4)]
            ]
        );
    }

    #[test]
    fn test_square_matrix_dimension() {
        let matrix = SquareMatrix::new(
            2,
            vec![
                vec![Integer::new(1), Integer::new(2)],
                vec![Integer::new(3), Integer::new(4)],
            ],
        );
        assert_eq!(matrix.dimension(), 2);
    }

    #[test]
    fn test_square_matrix_data() {
        let matrix = SquareMatrix::new(
            2,
            vec![
                vec![Integer::new(1), Integer::new(2)],
                vec![Integer::new(3), Integer::new(4)],
            ],
        );
        assert_eq!(
            matrix.data(),
            &vec![
                vec![Integer::new(1), Integer::new(2)],
                vec![Integer::new(3), Integer::new(4)]
            ]
        );
    }

    #[test]
    fn build_matrix_from_function_should_not_fail() {
        let matrix = SquareMatrix::from_fn(3, |i, j| Rational::from((i + j) as isize));
        let expected = SquareMatrix::new(
            3,
            vec![
                vec![Rational::from(0), Rational::from(1), Rational::from(2)],
                vec![Rational::from(1), Rational::from(2), Rational::from(3)],
                vec![Rational::from(2), Rational::from(3), Rational::from(4)],
            ],
        );

        pretty_assertions::assert_eq!(expected, matrix)
    }
}
