use std::str::FromStr;

use crate::{
    matrix::{
        parser::parse_matrix,
        traits::{
            ArithmeticallyOperable, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Identity,
            Matrix, Zero,
        },
    },
    result::{MathError, Result},
};

use super::f32::MatrixF32;

impl ArithmeticallyOperable<Result<MatrixF32>> for MatrixF32 {}

impl FromStr for MatrixF32 {
    type Err = MathError;

    /// Performs the conversion from a string to the matrix, with default tolerance 1e-12
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        parse_matrix(s, 1e-12)
    }
}

impl PartialEq for MatrixF32 {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

impl Zero for MatrixF32 {
    fn zero(rows: usize, columns: usize, tolerance: f32) -> Self {
        MatrixF32::new(vec![vec![f32::zero(0, 0, 0.0); columns]; rows], tolerance).unwrap()
    }

    fn is_zero(&self) -> bool {
        self.content
            .iter()
            .all(|row| row.iter().all(|x| x.is_zero()))
    }
}

impl Identity for MatrixF32 {
    fn id(dimensions: usize, tolerance: f32) -> Self {
        let mut result = vec![vec![f32::zero(0, 0, 0.0); dimensions]; dimensions];
        for i in 0..dimensions {
            result[i][i] = f32::id(0, 0.0);
        }
        MatrixF32::new(result, tolerance).unwrap()
    }
}

impl CheckedAdd for MatrixF32 {
    type Output = Result<MatrixF32>;
    fn checked_add(&self, rhs: &Self) -> Self::Output {
        if self.rows() != rhs.rows() || self.columns() != rhs.columns() {
            return Err(MathError::MatrixError(
                "Matrices must be of the same size".to_string(),
            ));
        } else {
            let mut result: Vec<Vec<f32>> = vec![];
            for i in 0..self.rows() {
                let mut result_row: Vec<f32> = vec![];
                for j in 0..self.columns() {
                    let left = self.get(i, j)?.to_owned();
                    let right = rhs.get(i, j)?.to_owned();
                    result_row.push(left.checked_add(&right)?)
                }
                result.push(result_row)
            }
            MatrixF32::new(result, self.tolerance())
        }
    }
}

impl CheckedSub for MatrixF32 {
    type Output = Result<MatrixF32>;

    fn checked_sub(&self, rhs: &Self) -> Self::Output {
        if self.rows() != rhs.rows() || self.columns() != rhs.columns() {
            return Err(MathError::MatrixError(
                "Matrices must be of the same size".to_string(),
            ));
        } else {
            let mut result: Vec<Vec<f32>> = vec![];
            for i in 0..self.rows() {
                let mut result_row: Vec<f32> = vec![];
                for j in 0..self.columns() {
                    let left = self.get(i, j)?.to_owned();
                    let right = rhs.get(i, j)?.to_owned();
                    result_row.push(left - right)
                }
                result.push(result_row)
            }
            MatrixF32::new(result, self.tolerance())
        }
    }
}

impl CheckedMul for MatrixF32 {
    type Output = Result<MatrixF32>;

    fn checked_mul(&self, rhs: &Self) -> Self::Output {
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
                let mut sum = f32::from_str("0")
                    .map_err(|_| MathError::MatrixError("Cannot build T from 0".to_string()))?;
                for k in 0..self.columns() {
                    sum = sum + self.get(i, k)?.clone() * rhs.get(k, j)?.clone()
                }
                row.push(sum);
            }
            result.push(row)
        }
        MatrixF32::new(result, self.tolerance())
    }
}

impl CheckedDiv for MatrixF32 {
    type Output = Result<MatrixF32>;

    fn checked_div(&self, rhs: &Self) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::matrix::float::f32::MatrixF32;
    use crate::matrix::traits::{CheckedAdd, CheckedMul, CheckedSub};
    use crate::matrix_f32;

    #[test]
    fn add_2x2_f32() {
        let mat_a = matrix_f32!("{{1,1},{1,1}}").unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}}").unwrap();
        let computed = mat_a.checked_add(&mat_b).unwrap();
        let expected = matrix_f32!("{{3,3},{3,3}}").unwrap();

        pretty_assertions::assert_eq!(computed, expected)
    }

    #[test]
    fn add_3x5_f32() {
        let mat_a = matrix_f32!("{{1,1,1,1,1}, {2,2,2,2,2}, {3,3,3,3,3}}").unwrap();
        let mat_b = matrix_f32!("{ {3,3,3,3,3},{2,2,2,2,2},  {1,1,1,1,1}}").unwrap();
        let computed = mat_a.checked_add(&mat_b).unwrap();
        let expected = matrix_f32!("{{4,4,4,4,4},{4,4,4,4,4},{4,4,4,4,4}}").unwrap();

        pretty_assertions::assert_eq!(computed, expected)
    }

    #[test]
    fn add_different_rows_should_fail() {
        let mat_a = matrix_f32!("{{1,1},{1,1}}").unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}, {2,2}}").unwrap();
        let computed = mat_a.checked_add(&mat_b);
        assert!(computed.is_err())
    }

    #[test]
    fn add_different_cols_should_fail() {
        let mat_a = matrix_f32!("{{1,1,1},{1,1,1}}").unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}}").unwrap();
        let computed = mat_a.checked_add(&mat_b);
        assert!(computed.is_err())
    }

    #[test]
    fn sub_2x2_f32() {
        let mat_a = matrix_f32!("{{1,1},{1,1}}").unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}}").unwrap();
        let computed = mat_a.checked_sub(&mat_b).unwrap();
        let expected = matrix_f32!("{{-1,-1},{-1,-1}}").unwrap();

        pretty_assertions::assert_eq!(computed, expected)
    }

    #[test]
    fn sub_3x5_f32() {
        let mat_a = matrix_f32!("{{1,1,1,1,1}, {2,2,2,2,2}, {3,3,3,3,3}}").unwrap();
        let mat_b = matrix_f32!("{ {3,3,3,3,3},{2,2,2,2,2},  {1,1,1,1,1}}").unwrap();
        let computed = mat_a.checked_sub(&mat_b).unwrap();
        let expected = matrix_f32!("{{-2,-2,-2,-2,-2},{0,0,0,0,0},{2,2,2,2,2}}").unwrap();

        pretty_assertions::assert_eq!(computed, expected)
    }

    #[test]
    fn sub_different_rows_should_fail() {
        let mat_a = matrix_f32!("{{1,1},{1,1}}").unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}, {2,2}}").unwrap();
        let computed = mat_a.checked_sub(&mat_b);
        assert!(computed.is_err())
    }

    #[test]
    fn sub_different_cols_should_fail() {
        let mat_a = matrix_f32!("{{1,1,1},{1,1,1}}").unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}}").unwrap();
        let computed = mat_a.checked_sub(&mat_b);
        assert!(computed.is_err())
    }

    #[test]
    fn mul_2x2_f32() {
        let mat_a = matrix_f32!("{{1,1},{1,1}}").unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}}").unwrap();
        let computed = mat_a.checked_mul(&mat_b).unwrap();
        let expected = matrix_f32!("{{4,4},{4,4}}").unwrap();

        pretty_assertions::assert_eq!(computed, expected)
    }

    #[test]
    fn mul_3x5x2_f32() {
        let mat_a = matrix_f32!("{{1,2,1,2,1}, {-1,2,-3,2,1}, {0,1,-3,2,1}}").unwrap();
        let mat_b = matrix_f32!("{{1,1}, {2,2}, {-1,-1}, {-2,-2}, {0,1}}").unwrap();
        let computed = mat_a.checked_mul(&mat_b).unwrap();
        let expected = matrix_f32!("{{0,1},{2,3},{1,2}}").unwrap();

        pretty_assertions::assert_eq!(computed, expected)
    }
}
