mod arith;
mod display;
pub mod macros;
mod parser;

use crate::result::{MathError, Result};
use std::fmt::Display;

use self::parser::parse_matrix;

// TODO: remove this
pub use self::parser::serialize_matrix;

use super::traits::{ArithmeticallyOperable, Matrix};

#[derive(Debug, Clone)]
pub struct GenericMatrix<T: ArithmeticallyOperable<T> + Display> {
    content: Vec<Vec<T>>,
}

impl<T: ArithmeticallyOperable<T> + Display> Matrix<T> for GenericMatrix<T> {
    fn columns(&self) -> usize {
        self.content
            .iter()
            .next()
            .map(|row| row.len())
            .expect("There is no row") // TODO: Arrange this in a better way
    }

    fn rows(&self) -> usize {
        self.content.len()
    }

    fn is_square(&self) -> bool {
        self.columns() == self.rows()
    }

    fn get(&self, row: usize, column: usize) -> Result<&T> {
        if row > self.rows() || column > self.columns() {
            return Err(MathError::MatrixError(format!("Cannot get element from position ({row},{column}) if the matrix has {}x{} dimensions!", self.rows(), self.columns())));
        }
        let Some(matrix_row) = self.content.get(row) else {
            return Err(MathError::MatrixError(format!("Cannot get row {row} if the matrix has {}x{} dimensions!", self.rows(), self.columns())));
        };
        matrix_row.get(column).ok_or(MathError::MatrixError(format!(
            "Cannot get element from position ({row},{column}) if the matrix has {}x{} dimensions!",
            self.rows(),
            self.columns()
        )))
    }

    fn get_mut(&mut self, row: usize, column: usize) -> Result<&mut T> {
        if row > self.rows() || column > self.columns() {
            return Err(MathError::MatrixError(format!("Cannot get element from position ({row},{column}) if the matrix has {}x{} dimensions!", self.rows(), self.columns())));
        }
        let rows = self.rows();
        let columns = self.columns();
        let Some(matrix_row) = self.content.get_mut(row) else {
            return Err(MathError::MatrixError(format!("Cannot get row {row} if the matrix has {}x{} dimensions!", rows, columns)));
        };
        matrix_row
            .get_mut(column)
            .ok_or(MathError::MatrixError(format!(
            "Cannot get element from position ({row},{column}) if the matrix has {}x{} dimensions!",
            rows,
            columns
        )))
    }

    fn set(&mut self, row: usize, column: usize, value: T) -> Result<()> {
        *self.get_mut(row, column)? = value;
        Ok(())
    }
}

impl<T: ArithmeticallyOperable<T> + Display> GenericMatrix<T> {
    pub fn new(content: Vec<Vec<T>>) -> Result<Self> {
        let mut content_iter = content.iter();
        if let Some(length) = content_iter.next().map(|row| row.len()) {
            while let Some(next_length) = content_iter.next().map(|row| row.len()) {
                if length != next_length {
                    return Err(MathError::MatrixError(
                        "Cannot build matrix from rows with different lenght".to_string(),
                    ));
                }
            }
        }
        Ok(Self { content })
    }

    pub fn zero_matrix_sized(rows: usize, columns: usize) -> Self {
        let mut matrix = Vec::with_capacity(rows);
        for _ in 0..rows {
            let row = Vec::with_capacity(columns);
            matrix.push(row)
        }
        GenericMatrix { content: matrix }
    }
}

impl<T: ArithmeticallyOperable<T> + Display> TryFrom<&str> for GenericMatrix<T> {
    fn try_from(value: &str) -> Result<Self> {
        parse_matrix(value)
    }

    type Error = MathError;
}

#[cfg(test)]
mod test {
    use crate::{matrix::traits::Matrix, matrix_usize};

    use super::GenericMatrix;
    use pretty_assertions;

    #[test]
    fn get_1() {
        let matrix = GenericMatrix::new(vec![vec![1, 2], vec![3, 4]]).unwrap();
        let item = matrix.get(0, 1).unwrap();
        pretty_assertions::assert_eq!(item.clone(), 2)
    }

    #[test]
    fn get_2() {
        let matrix = GenericMatrix::new(vec![
            vec![1.1, 2.2, 3.3],
            vec![4.4, 5.5, 6.6],
            vec![7.7, 8.8, 9.9],
        ])
        .unwrap();
        vec![((0, 0), 1.1), ((0, 2), 3.3), ((1, 1), 5.5), ((2, 0), 7.7)]
            .into_iter()
            .for_each(|item| {
                pretty_assertions::assert_eq!(
                    matrix.get(item.0 .0, item.0 .1).unwrap().clone(),
                    item.1
                )
            })
    }

    #[test]
    fn equal_matrices() {
        let matrix_a = GenericMatrix::new(vec![vec![1, 1], vec![2, 2]])
            .expect("Should've been able to built this matrix");
        let matrix_b = GenericMatrix::new(vec![vec![1, 1], vec![2, 2]])
            .expect("Should've been able to built this matrix");
        pretty_assertions::assert_eq!(matrix_a, matrix_b)
    }

    #[test]
    fn create_matrix_from_macro_1() {
        let matrix = matrix_usize!("{{1,2},{3,4}}")
            .expect("Should have been able to build matrix from macro");
        println!("{matrix}");
        let other = GenericMatrix::<usize>::try_from("{{1,2},{3,4}}").expect("asdf");
        pretty_assertions::assert_eq!(matrix, other)
    }

    #[test]
    fn sum_with_macro() {
        let result = (matrix_usize!("{{1,1},{1,1}}").expect("asdf")
            + matrix_usize!("{{2,2},{2,2}}").expect("asdf"))
        .expect("asdf");
        println!("{result}")
    }
}
