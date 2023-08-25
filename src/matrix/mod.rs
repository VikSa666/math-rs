pub mod error;

use crate::{
    field::{Field, FieldElement},
    traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Identity, Zero},
};

use self::error::MatrixError;

#[derive(Debug, Clone)]
pub struct Matrix<F: Field> {
    elements: Vec<Vec<F::Element>>,
}

impl<F: Field> Matrix<F> {
    pub fn rows(&self) -> usize {
        self.elements.len()
    }

    pub fn row_iter(&self) -> std::slice::Iter<'_, Vec<F::Element>> {
        self.elements.iter()
    }

    pub fn columns(&self) -> usize {
        self.elements.first().map_or(0, |row| row.len())
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&F::Element> {
        self.elements.get(row).and_then(|row| row.get(column))
    }

    pub fn set(&mut self, row: usize, column: usize, value: F::Element) {
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

    pub fn is_symmetric(&self) -> bool {
        // TODO: Set up the tolerance correctly
        self.is_square() && self.eq_with_tolerance(&self.transpose(), 1e-12)
    }

    pub fn swap_rows(&mut self, row1: usize, row2: usize) {
        if let Some(row1) = self.elements.get_mut(row1) {
            if let Some(row2) = self.elements.get_mut(row2) {
                row1.swap_with_slice(row2);
            }
        }
    }

    pub fn swap_columns(&mut self, column1: usize, column2: usize) {
        for row in self.elements.iter_mut() {
            row.swap(column1, column2);
        }
    }
}

impl<F: Field> TryFrom<Vec<Vec<F::Element>>> for Matrix<F> {
    type Error = MatrixError;

    fn try_from(value: Vec<Vec<F::Element>>) -> Result<Self, Self::Error> {
        let Some(first_row) = value.first() else {
            return Ok(Self::default());
        };
        if value.iter().any(|row| row.len() != first_row.len()) {
            return Err(MatrixError::InvalidNumberOfColumns);
        }
        Ok(Self { elements: value })
    }
}

impl<F: Field> Default for Matrix<F> {
    fn default() -> Self {
        Self {
            elements: Default::default(),
        }
    }
}

impl<F: Field> FieldElement for Matrix<F> {
    fn eq_with_tolerance(&self, other: &Self, tolerance: f32) -> bool {
        if self.rows() != other.rows() || self.columns() != other.columns() {
            return false;
        }
        for (row1, row2) in self.elements.iter().zip(other.elements.iter()) {
            for (element1, element2) in row1.iter().zip(row2.iter()) {
                if !element1.eq_with_tolerance(element2, tolerance) {
                    return false;
                }
            }
        }
        true
    }
}

impl<F: Field> Identity for Matrix<F> {
    fn id(_dimensions: usize, _tolerance: f32) -> Self {
        todo!()
    }
}

impl<F: Field> Zero for Matrix<F> {
    fn zero(rows: usize, columns: usize, tolerance: f32) -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}

impl<F: Field> CheckedMul for Matrix<F> {
    type Output = Result<Self, MatrixError>;

    fn checked_mul(&self, rhs: &Self) -> Self::Output {
        todo!()
    }
}

impl<F: Field> CheckedDiv for Matrix<F> {
    type Output = Result<Self, MatrixError>;

    fn checked_div(&self, rhs: &Self) -> Self::Output {
        todo!()
    }
}

impl<F: Field> CheckedAdd for Matrix<F> {
    type Output = Result<Self, MatrixError>;

    fn checked_add(&self, rhs: &Self) -> Self::Output {
        todo!()
    }
}

impl<F: Field> CheckedSub for Matrix<F> {
    type Output = Result<Self, MatrixError>;

    fn checked_sub(&self, rhs: &Self) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::field::rationals::Rationals;

    use super::*;

    #[test]
    fn matrix_try_from_should_fail() {
        let matrix = Matrix::<Rationals>::try_from(vec![vec![1.0, 2.0], vec![3.0]]);
        assert_eq!(matrix.err(), Some(MatrixError::InvalidNumberOfColumns));
    }

    #[test]
    fn matrix_try_from_should_not_fail() {
        let matrix = Matrix::<Rationals>::try_from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        assert_eq!(
            matrix.unwrap().elements,
            vec![vec![1.0, 2.0], vec![3.0, 4.0]]
        );
    }
}
