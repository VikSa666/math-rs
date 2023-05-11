use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

use crate::result::{MathError, Result};

use super::{parser::parse_matrix, ArithmeticallyOperable, GenericMatrix, Matrix};

impl<T> ArithmeticallyOperable<Result<GenericMatrix<T>>> for GenericMatrix<T> where
    T: ArithmeticallyOperable<T> + Display
{
}

impl<T> FromStr for GenericMatrix<T>
where
    T: ArithmeticallyOperable<T> + Display,
{
    type Err = MathError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        parse_matrix(s)
    }
}

impl<T> PartialEq for GenericMatrix<T>
where
    T: ArithmeticallyOperable<T> + Display,
{
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

impl<T> Add for GenericMatrix<T>
where
    T: ArithmeticallyOperable<T> + Display,
{
    type Output = Result<GenericMatrix<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.rows() != rhs.rows() || self.columns() != rhs.columns() {
            return Err(MathError::MatrixError(
                "Matrices must be of the same size".to_string(),
            ));
        } else {
            let mut result: Vec<Vec<T>> = vec![];
            for i in 0..self.rows() {
                let mut result_row: Vec<T> = vec![];
                for j in 0..self.columns() {
                    let left = self.get(i, j)?.to_owned();
                    let right = rhs.get(i, j)?.to_owned();
                    result_row.push(left + right)
                }
                result.push(result_row)
            }
            GenericMatrix::new(result)
        }
    }
}

impl<T> Sub for GenericMatrix<T>
where
    T: ArithmeticallyOperable<T> + Display,
{
    type Output = Result<GenericMatrix<T>>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.rows() != rhs.rows() || self.columns() != rhs.columns() {
            return Err(MathError::MatrixError(
                "Matrices must be of the same size".to_string(),
            ));
        } else {
            let mut result: Vec<Vec<T>> = vec![];
            for i in 0..self.rows() {
                let mut result_row: Vec<T> = vec![];
                for j in 0..self.columns() {
                    let left = self.get(i, j)?.to_owned();
                    let right = rhs.get(i, j)?.to_owned();
                    result_row.push(left - right)
                }
                result.push(result_row)
            }
            GenericMatrix::new(result)
        }
    }
}

impl<T> Mul for GenericMatrix<T>
where
    T: ArithmeticallyOperable<T> + Display,
{
    type Output = Result<GenericMatrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.columns() != rhs.rows() {
            return Err(MathError::MatrixError(format!(
                "Cannot multiply matrices of dimensions {}x{} and {}x{}",
                self.rows(),
                self.columns(),
                rhs.rows(),
                rhs.columns()
            )));
        }
        let rows = self.rows();
        let columns = rhs.columns();
        let mut result = Vec::with_capacity(rows);
        for i in 0..rows {
            let mut row = Vec::with_capacity(columns);
            for j in 0..columns {
                let mut sum: T = T::from_str("0")
                    .map_err(|_| MathError::MatrixError("Cannot build T from 0".to_string()))?;
                for k in 0..self.columns() {
                    sum = sum + self.get(i, k)?.clone() * rhs.get(k, j)?.clone()
                }
                row.push(sum);
            }
            result.push(row)
        }
        GenericMatrix::new(result)
    }
}

impl<T> Div for GenericMatrix<T>
where
    T: ArithmeticallyOperable<T> + Display,
{
    type Output = Result<GenericMatrix<T>>;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::matrix::GenericMatrix;
    use crate::matrix_f32;

    #[test]
    fn add_2x2_f32() {
        let mat_a = matrix_f32!("{{1,1},{1,1}}").unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}}").unwrap();
        let computed = (mat_a + mat_b).unwrap();
        let expected = matrix_f32!("{{3,3},{3,3}}").unwrap();

        pretty_assertions::assert_eq!(computed, expected)
    }

    #[test]
    fn add_3x5_f32() {
        let mat_a = matrix_f32!("{{1,1,1,1,1}, {2,2,2,2,2}, {3,3,3,3,3}}").unwrap();
        let mat_b = matrix_f32!("{ {3,3,3,3,3},{2,2,2,2,2},  {1,1,1,1,1}}").unwrap();
        let computed = (mat_a + mat_b).unwrap();
        let expected = matrix_f32!("{{4,4,4,4,4},{4,4,4,4,4},{4,4,4,4,4}}").unwrap();

        pretty_assertions::assert_eq!(computed, expected)
    }

    #[test]
    fn add_different_rows_should_fail() {
        let mat_a = matrix_f32!("{{1,1},{1,1}}").unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}, {2,2}}").unwrap();
        let computed = mat_a + mat_b;
        assert!(computed.is_err())
    }

    #[test]
    fn add_different_cols_should_fail() {
        let mat_a = matrix_f32!("{{1,1,1},{1,1,1}}").unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}}").unwrap();
        let computed = mat_a + mat_b;
        assert!(computed.is_err())
    }

    #[test]
    fn sub_2x2_f32() {
        let mat_a = matrix_f32!("{{1,1},{1,1}}").unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}}").unwrap();
        let computed = (mat_a - mat_b).unwrap();
        let expected = matrix_f32!("{{-1,-1},{-1,-1}}").unwrap();

        pretty_assertions::assert_eq!(computed, expected)
    }

    #[test]
    fn sub_3x5_f32() {
        let mat_a = matrix_f32!("{{1,1,1,1,1}, {2,2,2,2,2}, {3,3,3,3,3}}").unwrap();
        let mat_b = matrix_f32!("{ {3,3,3,3,3},{2,2,2,2,2},  {1,1,1,1,1}}").unwrap();
        let computed = (mat_a - mat_b).unwrap();
        let expected = matrix_f32!("{{-2,-2,-2,-2,-2},{0,0,0,0,0},{2,2,2,2,2}}").unwrap();

        pretty_assertions::assert_eq!(computed, expected)
    }

    #[test]
    fn sub_different_rows_should_fail() {
        let mat_a = matrix_f32!("{{1,1},{1,1}}").unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}, {2,2}}").unwrap();
        let computed = mat_a - mat_b;
        assert!(computed.is_err())
    }

    #[test]
    fn sub_different_cols_should_fail() {
        let mat_a = matrix_f32!("{{1,1,1},{1,1,1}}").unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}}").unwrap();
        let computed = mat_a - mat_b;
        assert!(computed.is_err())
    }

    #[test]
    fn mul_2x2_f32() {
        let mat_a = matrix_f32!("{{1,1},{1,1}}").unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}}").unwrap();
        let computed = (mat_a * mat_b).unwrap();
        let expected = matrix_f32!("{{4,4},{4,4}}").unwrap();

        pretty_assertions::assert_eq!(computed, expected)
    }

    #[test]
    fn mul_3x5x2_f32() {
        let mat_a = matrix_f32!("{{1,2,1,2,1}, {-1,2,-3,2,1}, {0,1,-3,2,1}}").unwrap();
        let mat_b = matrix_f32!("{{1,1}, {2,2}, {-1,-1}, {-2,-2}, {0,1}}").unwrap();
        let computed = (mat_a * mat_b).unwrap();
        let expected = matrix_f32!("{{0,1},{2,3},{1,2}}").unwrap();

        pretty_assertions::assert_eq!(computed, expected)
    }
}
