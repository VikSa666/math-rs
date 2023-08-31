pub mod display;
pub mod error;
pub mod ops;

use std::fmt::Display;

use num::Num;

use self::error::MatrixError;

#[derive(Debug, Clone)]
pub struct Matrix<F: Num + Clone + Display> {
    elements: Vec<Vec<F>>,
}

impl<F: Num + Clone + Display> Matrix<F> {
    pub fn rows(&self) -> usize {
        self.elements.len()
    }

    pub fn row_iter(&self) -> std::slice::Iter<'_, Vec<F>> {
        self.elements.iter()
    }

    pub fn columns(&self) -> usize {
        self.elements.first().map_or(0, |row| row.len())
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&F> {
        self.elements.get(row).and_then(|row| row.get(column))
    }

    pub fn set(&mut self, row: usize, column: usize, value: F) {
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

impl<F: Num + Clone + Display> TryFrom<Vec<Vec<F>>> for Matrix<F> {
    type Error = MatrixError;

    fn try_from(value: Vec<Vec<F>>) -> Result<Self, Self::Error> {
        let Some(first_row) = value.first() else {
            return Ok(Self::default());
        };
        if value.iter().any(|row| row.len() != first_row.len()) {
            return Err(MatrixError::InvalidNumberOfColumns);
        }
        Ok(Self { elements: value })
    }
}

impl<F: Num + Clone + Display> Default for Matrix<F> {
    fn default() -> Self {
        Self {
            elements: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use num::{Complex, FromPrimitive, Rational32};

    #[test]
    fn matrix_try_from_should_fail() {
        let matrix = Matrix::<Rational32>::try_from(vec![
            vec![
                Rational32::from_f32(1.0).unwrap(),
                Rational32::from_f32(2.0).unwrap(),
            ],
            vec![Rational32::from_f32(3.0).unwrap()],
        ]);
        assert_eq!(matrix.err(), Some(MatrixError::InvalidNumberOfColumns));
    }

    #[test]
    fn matrix_try_from_should_not_fail() {
        let matrix = Matrix::<Rational32>::try_from(vec![
            vec![
                Rational32::from_f32(1.0).unwrap(),
                Rational32::from_f32(2.0).unwrap(),
            ],
            vec![
                Rational32::from_f32(3.0).unwrap(),
                Rational32::from_f32(3.0).unwrap(),
            ],
        ]);
        assert_eq!(
            matrix.unwrap().elements,
            vec![
                vec![
                    Rational32::from_f32(1.0).unwrap(),
                    Rational32::from_f32(2.0).unwrap(),
                ],
                vec![
                    Rational32::from_f32(3.0).unwrap(),
                    Rational32::from_f32(3.0).unwrap(),
                ],
            ]
        );
    }

    #[test]
    fn matrix_complex_should_not_fail() {
        Matrix::<Complex<f32>>::try_from(vec![
            vec![Complex::<f32>::new(1., 1.), Complex::<f32>::new(1., 1.)],
            vec![Complex::<f32>::new(1., 1.), Complex::<f32>::new(1., 1.)],
        ])
        .unwrap();
    }
}
