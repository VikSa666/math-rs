pub mod ops;
pub mod parser;

use std::ops::{Index, IndexMut};

use crate::structures::Ring;

use super::{AsMatrix, MatrixError};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<R: Ring> {
    pub data: Vec<Vec<R>>,
}

impl<R: Ring> TryFrom<Vec<Vec<R>>> for Matrix<R> {
    type Error = MatrixError;

    fn try_from(value: Vec<Vec<R>>) -> Result<Self, Self::Error> {
        let Some(first_row) = value.first() else {
            return Ok(Self::default());
        };
        if value.iter().any(|row| row.len() != first_row.len()) {
            return Err(MatrixError::InvalidNumberOfColumns);
        }
        Ok(Self { data: value })
    }
}

impl<R: Ring> Default for Matrix<R> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<R: Ring> Index<(usize, usize)> for Matrix<R> {
    type Output = R;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<R: Ring> IndexMut<(usize, usize)> for Matrix<R> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

impl<R> AsMatrix<R> for Matrix<R>
where
    R: Ring + PartialOrd,
{
    fn data(&self) -> &Vec<Vec<R>> {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Vec<Vec<R>> {
        &mut self.data
    }

    fn with_capacity(rows: usize, columns: usize) -> Self {
        let mut elements = Vec::with_capacity(rows);
        for _ in 0..rows {
            let mut row = Vec::with_capacity(columns);
            for _ in 0..columns {
                row.push(R::zero(0, 0));
            }
            elements.push(row);
        }
        Self { data: elements }
    }

    fn rows(&self) -> usize {
        self.data.len()
    }

    fn row_iter(&self) -> std::slice::Iter<'_, Vec<R>> {
        self.data.iter()
    }

    fn columns(&self) -> usize {
        self.data.first().map_or(0, |row| row.len())
    }

    fn get(&self, row: usize, column: usize) -> Result<&R, MatrixError> {
        self.data
            .get(row)
            .and_then(|row| row.get(column))
            .ok_or(MatrixError::ElementNotFound(row, column))
    }

    fn get_mut(&mut self, row: usize, column: usize) -> Result<&mut R, MatrixError> {
        self.data
            .get_mut(row)
            .and_then(|row| row.get_mut(column))
            .ok_or(MatrixError::ElementNotFound(row, column))
    }

    fn set(&mut self, row: usize, column: usize, value: R) -> Result<(), MatrixError> {
        let element = self.get_mut(row, column)?;
        *element = value;
        Ok(())
    }

    fn transpose(&self) -> Self {
        let mut elements = Vec::with_capacity(self.columns());
        for column in 0..self.columns() {
            let mut new_row = Vec::with_capacity(self.rows());
            for row in self.data.iter() {
                new_row.push(row[column].clone());
            }
            elements.push(new_row);
        }
        Self { data: elements }
    }

    fn is_square(&self) -> bool {
        self.rows() == self.columns()
    }
}

#[cfg(test)]
mod test {

    use crate::{
        identities::One,
        structures::{complex::Complex, integers::Integer, rationals::Rational, reals::Real},
    };

    use super::*;

    #[test]
    fn matrix_try_from_should_fail() {
        let matrix = Matrix::<Rational<i32>>::try_from(vec![
            vec![
                Rational::<i32>::new(Integer::<i32>::new(1), Integer::one(0, 0)),
                Rational::<i32>::new(Integer::<i32>::new(2), Integer::one(0, 0)),
            ],
            vec![Rational::<i32>::new(
                Integer::<i32>::new(3),
                Integer::one(0, 0),
            )],
        ]);
        assert_eq!(matrix.err(), Some(MatrixError::InvalidNumberOfColumns));
    }

    #[test]
    fn rational_matrix_try_from_should_not_fail() {
        let matrix = Matrix::<Rational<i32>>::try_from(vec![
            vec![
                Rational::<i32>::new(Integer::<i32>::new(1), Integer::one(0, 0)),
                Rational::<i32>::new(Integer::<i32>::new(2), Integer::one(0, 0)),
            ],
            vec![
                Rational::<i32>::new(Integer::<i32>::new(3), Integer::one(0, 0)),
                Rational::<i32>::new(Integer::<i32>::new(3), Integer::one(0, 0)),
            ],
        ]);
        assert_eq!(
            matrix.unwrap().data,
            vec![
                vec![
                    Rational::<i32>::new(Integer::<i32>::new(1), Integer::one(0, 0)),
                    Rational::<i32>::new(Integer::<i32>::new(2), Integer::one(0, 0)),
                ],
                vec![
                    Rational::<i32>::new(Integer::<i32>::new(3), Integer::one(0, 0)),
                    Rational::<i32>::new(Integer::<i32>::new(3), Integer::one(0, 0)),
                ],
            ]
        );
    }

    #[test]
    fn integer_matrix_try_from_should_not_fail() {
        let matrix = Matrix::<Integer<isize>>::try_from(vec![
            vec![
                Integer::<isize>::new(1),
                Integer::<isize>::new(2),
                Integer::<isize>::new(3),
            ],
            vec![
                Integer::<isize>::new(4),
                Integer::<isize>::new(5),
                Integer::<isize>::new(6),
            ],
        ]);

        assert_eq!(
            matrix.unwrap().data,
            vec![
                vec![
                    Integer::<isize>::new(1),
                    Integer::<isize>::new(2),
                    Integer::<isize>::new(3),
                ],
                vec![
                    Integer::<isize>::new(4),
                    Integer::<isize>::new(5),
                    Integer::<isize>::new(6),
                ],
            ]
        );
    }

    #[test]
    fn real_matrix_try_from_should_not_fail() {
        let matrix = Matrix::<Real>::try_from(vec![
            vec![Real::new(1.), Real::new(2.), Real::new(3.)],
            vec![Real::new(4.), Real::new(5.), Real::new(6.)],
        ]);

        assert_eq!(
            matrix.unwrap().data,
            vec![
                vec![Real::new(1.), Real::new(2.), Real::new(3.)],
                vec![Real::new(4.), Real::new(5.), Real::new(6.)],
            ]
        );
    }

    #[test]
    fn complex_matrix_try_from_should_not_fail() {
        let matrix = Matrix::<Complex>::try_from(vec![
            vec![
                Complex::from((1., 1.)),
                Complex::from((2., 2.)),
                Complex::from((3., 3.)),
            ],
            vec![
                Complex::from((4., 4.)),
                Complex::from((5., 5.)),
                Complex::from((6., 6.)),
            ],
        ]);

        assert_eq!(
            matrix.unwrap().data,
            vec![
                vec![
                    Complex::from((1., 1.)),
                    Complex::from((2., 2.)),
                    Complex::from((3., 3.)),
                ],
                vec![
                    Complex::from((4., 4.)),
                    Complex::from((5., 5.)),
                    Complex::from((6., 6.)),
                ],
            ]
        );
    }
}
