pub mod display;
pub mod error;
pub mod ops;

use crate::structures::Ring;

use self::error::MatrixError;

#[derive(Debug, Clone)]
pub struct Matrix<R: Ring> {
    elements: Vec<Vec<R>>,
}

impl<R: Ring> Matrix<R> {
    pub fn rows(&self) -> usize {
        self.elements.len()
    }

    pub fn row_iter(&self) -> std::slice::Iter<'_, Vec<R>> {
        self.elements.iter()
    }

    pub fn columns(&self) -> usize {
        self.elements.first().map_or(0, |row| row.len())
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&R> {
        self.elements.get(row).and_then(|row| row.get(column))
    }

    pub fn set(&mut self, row: usize, column: usize, value: R) {
        if let Some(row) = self.elements.get_mut(row) {
            if let Some(element) = row.get_mut(column) {
                *element = value;
            }
        }
    }

    pub fn transpose(&self) -> Self {
        let mut elements = Vec::with_capacity(self.columns());
        for column in 0..self.columns() {
            let mut new_row = Vec::with_capacity(self.rows());
            for row in self.elements.iter() {
                new_row.push(row[column].clone());
            }
            elements.push(new_row);
        }
        Self { elements }
    }

    pub fn is_square(&self) -> bool {
        self.rows() == self.columns()
    }
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
        Ok(Self { elements: value })
    }
}

impl<R: Ring> Default for Matrix<R> {
    fn default() -> Self {
        Self {
            elements: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{
        identities::One,
        structures::{integers::Integer, rationals::Rational},
    };

    use super::*;

    #[test]
    fn matrix_try_from_should_fail() {
        let matrix = Matrix::<Rational<i32>>::try_from(vec![
            vec![
                Rational::<i32>::new(Integer::<i32>::new(1), Integer::one()),
                Rational::<i32>::new(Integer::<i32>::new(2), Integer::one()),
            ],
            vec![Rational::<i32>::new(Integer::<i32>::new(3), Integer::one())],
        ]);
        assert_eq!(matrix.err(), Some(MatrixError::InvalidNumberOfColumns));
    }

    #[test]
    fn rational_matrix_try_from_should_not_fail() {
        let matrix = Matrix::<Rational<i32>>::try_from(vec![
            vec![
                Rational::<i32>::new(Integer::<i32>::new(1), Integer::one()),
                Rational::<i32>::new(Integer::<i32>::new(2), Integer::one()),
            ],
            vec![
                Rational::<i32>::new(Integer::<i32>::new(3), Integer::one()),
                Rational::<i32>::new(Integer::<i32>::new(3), Integer::one()),
            ],
        ]);
        assert_eq!(
            matrix.unwrap().elements,
            vec![
                vec![
                    Rational::<i32>::new(Integer::<i32>::new(1), Integer::one()),
                    Rational::<i32>::new(Integer::<i32>::new(2), Integer::one()),
                ],
                vec![
                    Rational::<i32>::new(Integer::<i32>::new(3), Integer::one()),
                    Rational::<i32>::new(Integer::<i32>::new(3), Integer::one()),
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
            matrix.unwrap().elements,
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
}
