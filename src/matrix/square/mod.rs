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
    R: Ring + PartialOrd,
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

    /// Checks if any element of the diagonal is zero
    pub fn diagonal_is_zero(&self, tolerance: f32) -> bool {
        for row in 0..self.dimension() {
            if self.data[row][row].is_zero(tolerance) {
                return true;
            }
        }
        false
    }

    /// This function returns a matrix with the given `row` and `column` removed.
    ///
    /// ## Example
    ///
    /// Given the matrix
    /// ```txt
    /// 1 2 3
    /// 4 5 6
    /// 7 8 9
    /// ```
    /// and the parameters
    /// * `row = 1`
    /// * `column = 2`
    /// we would get the matrix
    /// ```txt
    /// 1 2
    /// 7 8
    /// ```
    ///
    /// ## Errors
    /// It returns an error whenever the `row` or `column` are out of bounds.
    ///
    /// ## Time complexity
    /// This function has a time complexity of `O(n^2)`.
    pub fn minor(&self, row: usize, column: usize) -> Result<Self, MatrixError> {
        if row >= self.dimension() {
            return Err(MatrixError::RowOutOfBounds(row));
        }
        if column >= self.dimension() {
            return Err(MatrixError::ColumnOutOfBounds(column));
        }
        let mut elements = Vec::with_capacity(self.dimension() - 1);
        for i in 0..self.dimension() {
            if i == row {
                continue;
            }
            let mut new_row = Vec::with_capacity(self.dimension() - 1);
            for j in 0..self.dimension() {
                if j == column {
                    continue;
                }
                new_row.push(self.data[i][j].clone());
            }
            elements.push(new_row);
        }
        Ok(Self {
            dimension: self.dimension() - 1,
            data: elements,
        })
    }

    /// Swaps the rows with 0 pivot element with the first row that has a non-zero pivot element.
    ///
    /// It takes a mutalbe reference of the matrix and mutates it. If at some point, the matrix
    /// has a column with all zero elements, it returns `false`. This will mean that the
    /// matrix is **singular**.
    ///
    /// ## Example
    ///
    /// Given the matrix
    /// ```txt
    /// 0 1 2
    /// 1 2 3
    /// 2 3 4
    /// ```
    /// we would get the matrix
    /// ```txt
    /// 1 2 3
    /// 0 1 2
    /// 2 3 4
    /// ```
    ///
    /// ## Time complexity
    /// This function has a time complexity of `O(n^2)`.
    pub fn swap_rows_with_0_pivot(&mut self, tolerance: f32) -> Result<bool, MatrixError> {
        for row in 0..self.dimension() {
            if self.data[row][row].is_zero(tolerance) {
                for row2 in row + 1..self.dimension() {
                    if !self.data[row2][row].is_zero(tolerance) {
                        self.swap_rows(row, row2)?;
                        return Ok(true);
                    }
                }
                return Ok(false);
            }
        }
        Ok(false)
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
            for element in row.iter() {
                output.push_str(&format!("{} ", element));
            }
            output.push_str("\n")
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

    #[test]
    fn minor_computation_should_not_fail() {
        let matrix = SquareMatrix::from_fn(3, |i, j| Rational::from((i + j) as isize));
        let expected = SquareMatrix::new(
            2,
            vec![
                vec![Rational::from(0), Rational::from(1)],
                vec![Rational::from(2), Rational::from(3)],
            ],
        );
        let computed = matrix.minor(1, 2).unwrap();
        pretty_assertions::assert_eq!(expected, computed);

        let huge_matrix = SquareMatrix::from_fn(100, |i, j| {
            if (i as isize - j as isize).abs() < 3 {
                Integer::from(1)
            } else {
                Integer::from(0)
            }
        });
        let expected = SquareMatrix::from_fn(99, |i, j| {
            if (i as isize - j as isize).abs() < 3 {
                Integer::from(1)
            } else {
                Integer::from(0)
            }
        });
        let start = std::time::Instant::now();
        let computed = huge_matrix.minor(0, 0).unwrap();
        let time = std::time::Instant::now() - start;
        pretty_assertions::assert_eq!(expected, computed);
        assert!(time.as_micros() < 1000);
    }

    #[test]
    fn swap_rows_with_0_pivot_should_not_fail() {
        let mut matrix = SquareMatrix::new(
            3,
            vec![
                vec![Integer::new(0), Integer::new(1), Integer::new(2)],
                vec![Integer::new(1), Integer::new(2), Integer::new(3)],
                vec![Integer::new(2), Integer::new(3), Integer::new(4)],
            ],
        );
        let expected = SquareMatrix::new(
            3,
            vec![
                vec![Integer::new(1), Integer::new(2), Integer::new(3)],
                vec![Integer::new(0), Integer::new(1), Integer::new(2)],
                vec![Integer::new(2), Integer::new(3), Integer::new(4)],
            ],
        );
        matrix.swap_rows_with_0_pivot(1e-12).unwrap();
        pretty_assertions::assert_eq!(expected, matrix);
    }
}
