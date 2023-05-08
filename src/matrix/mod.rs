mod arith;
mod display;
mod traits;

use crate::result::{MathError, Result};
use std::fmt::Display;

use self::traits::{ArithmeticOperation, Matrix};

#[derive(Debug)]
pub struct GenericMatrix<T: ArithmeticOperation<T> + Display> {
    rows: usize,
    columns: usize,
    content: Vec<T>,
}

impl<T: ArithmeticOperation<T> + Display> Matrix<T> for GenericMatrix<T> {
    fn columns(&self) -> usize {
        self.columns
    }

    fn rows(&self) -> usize {
        self.rows
    }

    fn is_square(&self) -> bool {
        self.columns == self.rows
    }

    fn get(&self, row: usize, column: usize) -> Result<&T> {
        if row > self.rows || column > self.columns {
            return Err(MathError::MatrixError(format!("Cannot get element from position ({row},{column}) if the matrix has {}x{} dimensions!", self.rows, self.columns)));
        }
        let target_position = (row - 1) * self.columns + (column - 1);
        self.content
            .get(target_position)
            .ok_or(MathError::MatrixError(format!(
                "Could not get position {target_position}"
            )))
    }

    fn set(&mut self, row: usize, column: usize, value: T) -> Result<()> {
        todo!()
    }
}

impl<T: ArithmeticOperation<T> + Display> GenericMatrix<T> {
    pub fn new(content: Vec<T>, rows: usize, columns: usize) -> Result<Self> {
        if content.len() != rows * columns {
            Err(MathError::MatrixError(format!(
                "Cannot build matrix of dimensions {rows}x{columns} with {} elements!",
                content.len()
            )))
        } else {
            Ok(Self {
                content,
                rows,
                columns,
            })
        }
    }

    pub fn get_mut<'a>(&'a mut self, row: usize, column: usize) -> Result<&'a mut T> {
        if row > self.rows || column > self.columns {
            return Err(MathError::MatrixError(format!("Cannot get element from position ({row},{column}) if the matrix has {}x{} dimensions!", self.rows, self.columns)));
        }
        let target_position = (row - 1) * self.columns + (column - 1);
        self.content
            .get_mut(target_position)
            .ok_or(MathError::MatrixError(format!(
                "Could not get position {target_position}"
            )))
    }
}

#[cfg(test)]
mod test {
    use crate::matrix::traits::Matrix;

    use super::GenericMatrix;
    use pretty_assertions;

    #[test]
    fn get_1() {
        let matrix = GenericMatrix::new(vec![1, 2, 3, 4], 2, 2).unwrap();
        let item = matrix.get(1, 2).unwrap();
        pretty_assertions::assert_eq!(item.clone(), 2)
    }

    #[test]
    fn get_2() {
        let matrix =
            GenericMatrix::new(vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9], 3, 3).unwrap();
        vec![((1, 1), 1.1), ((1, 3), 3.3), ((2, 2), 5.5), ((3, 1), 7.7)]
            .into_iter()
            .for_each(|item| {
                pretty_assertions::assert_eq!(
                    matrix.get(item.0 .0, item.0 .1).unwrap().clone(),
                    item.1
                )
            })
    }
}
