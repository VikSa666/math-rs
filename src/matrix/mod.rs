pub mod display;
mod error;
use std::{fmt::Display, str::FromStr};

pub use error::MatrixError;

use crate::structures::Ring;

pub mod generic;
pub mod square;

pub trait AsMatrix<R>: TryFrom<Vec<Vec<R>>> + Default + FromStr + Display + Clone
where
    R: Ring + PartialOrd,
{
    fn data(&self) -> &Vec<Vec<R>>;
    fn data_mut(&mut self) -> &mut Vec<Vec<R>>;
    fn with_capacity(rows: usize, columns: usize) -> Self;
    fn rows(&self) -> usize;
    fn columns(&self) -> usize;
    fn row_iter(&self) -> std::slice::Iter<'_, Vec<R>>;
    fn get(&self, row: usize, column: usize) -> Result<&R, MatrixError>;
    fn get_mut(&mut self, row: usize, column: usize) -> Result<&mut R, MatrixError>;
    fn set(&mut self, row: usize, column: usize, value: R) -> Result<(), MatrixError>;
    fn transpose(&self) -> Self;
    fn is_square(&self) -> bool {
        self.rows() == self.columns()
    }
    fn swap_rows(&mut self, row1: usize, row2: usize) -> Result<(), MatrixError> {
        if row1 == row2 {
            return Ok(());
        }
        if row1 >= self.rows() || row2 >= self.rows() {
            return Err(MatrixError::InvalidNumberOfRows);
        }
        self.data_mut().swap(row1, row2);
        Ok(())
    }

    /// Returns a brand new matrix resulting from gaussian elimination.
    ///
    /// ## Parameters
    /// - `tolerance`: The tolerance used to determine if a number is zero.
    ///
    /// ## Example
    ///
    /// If you have the matrix
    /// ```txt
    ///     1   2   3
    /// M = 4   5   6
    ///     7   8   9
    /// ```
    /// and tolerance is 1e-6, then the result will be the matrix
    /// ```txt
    ///     1   2   3
    /// M'= 0   0   0
    ///     0   0   0
    /// ```
    ///
    /// ## Complexity
    /// The complexity of this algorithm is _O(n^3)_.
    fn gaussian_elimination(&self, tolerance: f32) -> Result<Self, MatrixError> {
        let mut matrix = self.clone();
        let mut i = 0;
        let mut j = 0;
        while i < matrix.rows() && j < matrix.columns() {
            let mut max_row = i;
            for k in i + 1..matrix.rows() {
                if matrix.data()[k][j].abs_value() > matrix.data()[max_row][j].abs_value() {
                    max_row = k;
                }
            }
            if matrix.data()[max_row][j].is_zero(tolerance) {
                j += 1;
            } else {
                matrix.swap_rows(i, max_row)?;
                for k in i + 1..matrix.rows() {
                    let factor = matrix.data()[k][j].clone() / matrix.data()[i][j].clone();
                    matrix.data_mut()[k][j] = R::zero();
                    for l in j + 1..matrix.columns() {
                        let new_value = matrix.data()[k][l].clone()
                            - matrix.data()[i][l].clone() * factor.clone();
                        matrix.data_mut()[k][l] = new_value;
                    }
                }
                i += 1;
                j += 1;
            }
        }
        Ok(matrix)
    }

    /// Returns a brand new matrix that is equal to the original matrix, but with the column
    /// you specify removed.
    ///
    /// ## Example
    ///
    /// If you have the matrix
    /// ```txt
    ///     1   2   3
    /// M = 4   5   6
    ///     7   8   9
    /// ```
    /// and column is 1, then the result will be the matrix
    /// ```txt
    ///     1   3
    /// R = 4   6
    ///     7   9
    /// ```
    ///
    /// ## Errors
    ///
    /// Returns an error if you give a column index out of bounds.
    fn remove_column(&self, column: usize) -> Result<Self, MatrixError> {
        if column >= self.columns() {
            return Err(MatrixError::ColumnOutOfBounds(column));
        }

        let mut new_matrix = Self::with_capacity(self.rows(), self.columns() - 1);

        for i in 0..self.rows() {
            for j in 0..column {
                new_matrix.data_mut()[i][j] = self.data()[i][j].clone();
            }
            for j in column + 1..self.columns() {
                new_matrix.data_mut()[i][j - 1] = self.data()[i][j].clone();
            }
        }

        Ok(new_matrix)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        equality::Equals,
        matrix::{error::MatrixError, generic::Matrix, AsMatrix},
        structures::{integers::Integer, rationals::Rational, reals::Real, Ring},
    };

    #[test]
    fn remove_columns_should_not_panic() {
        let matrix = Matrix::<Integer<i32>>::try_from(vec![
            vec![
                Integer::new(1),
                Integer::new(2),
                Integer::new(3),
                Integer::new(4),
            ],
            vec![
                Integer::new(2),
                Integer::new(1),
                Integer::new(4),
                Integer::new(3),
            ],
            vec![
                Integer::new(4),
                Integer::new(3),
                Integer::new(2),
                Integer::new(1),
            ],
            vec![
                Integer::new(3),
                Integer::new(4),
                Integer::new(1),
                Integer::new(2),
            ],
        ])
        .unwrap();

        let removed_column_matrix = matrix.remove_column(2).unwrap();
        let expected = Matrix::<Integer<i32>>::try_from(vec![
            vec![Integer::new(1), Integer::new(2), Integer::new(4)],
            vec![Integer::new(2), Integer::new(1), Integer::new(3)],
            vec![Integer::new(4), Integer::new(3), Integer::new(1)],
            vec![Integer::new(3), Integer::new(4), Integer::new(2)],
        ])
        .unwrap();

        pretty_assertions::assert_eq!(expected, removed_column_matrix)
    }

    #[test]
    fn swap_rows() {
        let mut matrix = Matrix::<Integer<i32>>::try_from(vec![
            vec![
                Integer::new(1),
                Integer::new(2),
                Integer::new(3),
                Integer::new(4),
            ],
            vec![
                Integer::new(2),
                Integer::new(1),
                Integer::new(4),
                Integer::new(3),
            ],
            vec![
                Integer::new(4),
                Integer::new(3),
                Integer::new(2),
                Integer::new(1),
            ],
            vec![
                Integer::new(3),
                Integer::new(4),
                Integer::new(1),
                Integer::new(2),
            ],
        ])
        .unwrap();
        matrix.swap_rows(0, 1).unwrap();
        matrix.swap_rows(1, 2).unwrap();
        matrix.swap_rows(2, 3).unwrap();
        assert_eq!(
            *matrix.data(),
            vec![
                vec![
                    Integer::new(2),
                    Integer::new(1),
                    Integer::new(4),
                    Integer::new(3),
                ],
                vec![
                    Integer::new(4),
                    Integer::new(3),
                    Integer::new(2),
                    Integer::new(1),
                ],
                vec![
                    Integer::new(3),
                    Integer::new(4),
                    Integer::new(1),
                    Integer::new(2),
                ],
                vec![
                    Integer::new(1),
                    Integer::new(2),
                    Integer::new(3),
                    Integer::new(4),
                ],
            ]
        );
    }

    struct TestCase<'a> {
        id: &'a str,
        matrix: &'a str,
        expected: &'a str,
    }

    const TOLERANCE: f32 = 1e-12;

    fn perform_test<'a, R: Ring + PartialOrd>(
        test: TestCase<'a>,
        builder: fn(&str) -> Result<Matrix<R>, MatrixError>,
    ) {
        let matrix = builder(test.matrix).unwrap();
        let expected = builder(test.expected).unwrap();
        let reduced = matrix.gaussian_elimination(TOLERANCE).unwrap();
        assert!(
            reduced.equals(&expected, 1e-6),
            "Test case: {} failed. Expected\n{expected}but got\n{reduced}",
            test.id,
            expected = expected,
            reduced = reduced
        );
    }

    #[test]
    fn gaussian_elimination_with_real_matrix() {
        vec![
            TestCase {
                id: "Simple 2x2",
                matrix: "{{1,2},{3,4}}",
                expected: "{{3,4},{0,0.6666667}}",
            },
            TestCase {
                id: "Simple 3x3",
                matrix: "{{1,2,3},{4,5,6},{7,8,9}}",
                expected: "{{7,8,9},{0,+0.8571428,+1.7142856},{0,0,0}}",
            },
            TestCase {
                id: "More rows than columns",
                matrix: "{{1,2,3},{4,5,6},{7,8,9},{10,11,12}}",
                expected: "{{10,11,12},{0,+0.9,+1.8},{0,0,0},{0,0,0}}",
            },
            TestCase {
                id: "More columns than rows",
                matrix: "{{1,2},{3,4},{5,6},{7,8}}",
                expected: "{{7,8},{0,0.8571428},{0,0},{0,0}}",
            },
        ]
        .into_iter()
        .for_each(|test| perform_test(test, Matrix::<Real>::from_str));
    }

    #[test]
    fn gaussian_elimination_with_rational_matrix() {
        vec![
            TestCase {
                id: "Simple 2x2",
                matrix: "{{1,2},{3,4}}",
                expected: "{{3,4},{0,2/3}}",
            },
            TestCase {
                id: "Simple 3x3",
                matrix: "{{1,2,3},{4,5,6},{7,8,9}}",
                expected: "{{7,8,9},{0,6/7,12/7},{0,0,0}}",
            },
            TestCase {
                id: "More rows than columns",
                matrix: "{{1,2,3},{4,5,6},{7,8,9},{10,11,12}}",
                expected: "{{10,11,12},{0,9/10,9/5},{0,0,0},{0,0,0}}",
            },
            TestCase {
                id: "More columns than rows",
                matrix: "{{1,2},{3,4},{5,6},{7,8}}",
                expected: "{{7,8},{0,6/7},{0,0},{0,0}}",
            },
        ]
        .into_iter()
        .for_each(|test| perform_test(test, Matrix::<Rational<i32>>::from_str))
    }
}
