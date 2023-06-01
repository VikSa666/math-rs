use std::str::FromStr;

use crate::{
    matrix::{Matrix, Parseable},
    result::{MathError, Result},
    traits::{ArithmeticallyOperable, CheckedAdd, CheckedMul, CheckedSub, Identity, Zero},
};

use super::f32::MatrixF32;

impl ArithmeticallyOperable for MatrixF32 {}

impl FromStr for MatrixF32 {
    type Err = MathError;

    /// Performs the conversion from a string to the matrix, with default tolerance 1e-12
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::parse(s, 1e-12)
    }
}

impl PartialEq for MatrixF32 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.rows() {
            for j in 0..self.columns() {
                if (self.get(i, j).unwrap() - other.get(i, j).unwrap()).abs() > self.tolerance() {
                    return false;
                }
            }
        }
        true
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

#[cfg(test)]
mod test {
    use crate::matrix::float::f32::MatrixF32;
    use crate::matrix::Parseable;
    use crate::matrix_f32;
    use crate::traits::{CheckedAdd, CheckedMul, CheckedSub};

    const TOL: f32 = 1e-12;

    #[test]
    fn add_2x2_f32() {
        let mat_a = matrix_f32!("{{1,1},{1,1}}", TOL).unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}}", TOL).unwrap();
        let computed = mat_a.checked_add(&mat_b).unwrap();
        let expected = matrix_f32!("{{3,3},{3,3}}", TOL).unwrap();

        pretty_assertions::assert_eq!(computed, expected)
    }

    #[test]
    fn add_3x5_f32() {
        let mat_a = matrix_f32!("{{1,1,1,1,1}, {2,2,2,2,2}, {3,3,3,3,3}}", TOL).unwrap();
        let mat_b = matrix_f32!("{ {3,3,3,3,3},{2,2,2,2,2},  {1,1,1,1,1}}", TOL).unwrap();
        let computed = mat_a.checked_add(&mat_b).unwrap();
        let expected = matrix_f32!("{{4,4,4,4,4},{4,4,4,4,4},{4,4,4,4,4}}", TOL).unwrap();

        pretty_assertions::assert_eq!(computed, expected)
    }

    #[test]
    fn add_different_rows_should_fail() {
        let mat_a = matrix_f32!("{{1,1},{1,1}}", TOL).unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}, {2,2}}", TOL).unwrap();
        let computed = mat_a.checked_add(&mat_b);
        assert!(computed.is_err())
    }

    #[test]
    fn add_different_cols_should_fail() {
        let mat_a = matrix_f32!("{{1,1,1},{1,1,1}}", TOL).unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}}", TOL).unwrap();
        let computed = mat_a.checked_add(&mat_b);
        assert!(computed.is_err())
    }

    #[test]
    fn sub_2x2_f32() {
        let mat_a = matrix_f32!("{{1,1},{1,1}}", TOL).unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}}", TOL).unwrap();
        let computed = mat_a.checked_sub(&mat_b).unwrap();
        let expected = matrix_f32!("{{-1,-1},{-1,-1}}", TOL).unwrap();

        pretty_assertions::assert_eq!(computed, expected)
    }

    #[test]
    fn sub_3x5_f32() {
        let mat_a = matrix_f32!("{{1,1,1,1,1}, {2,2,2,2,2}, {3,3,3,3,3}}", TOL).unwrap();
        let mat_b = matrix_f32!("{ {3,3,3,3,3},{2,2,2,2,2},  {1,1,1,1,1}}", TOL).unwrap();
        let computed = mat_a.checked_sub(&mat_b).unwrap();
        let expected = matrix_f32!("{{-2,-2,-2,-2,-2},{0,0,0,0,0},{2,2,2,2,2}}", TOL).unwrap();

        pretty_assertions::assert_eq!(computed, expected)
    }

    #[test]
    fn sub_different_rows_should_fail() {
        let mat_a = matrix_f32!("{{1,1},{1,1}}", TOL).unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}, {2,2}}", TOL).unwrap();
        let computed = mat_a.checked_sub(&mat_b);
        assert!(computed.is_err())
    }

    #[test]
    fn sub_different_cols_should_fail() {
        let mat_a = matrix_f32!("{{1,1,1},{1,1,1}}", TOL).unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}}", TOL).unwrap();
        let computed = mat_a.checked_sub(&mat_b);
        assert!(computed.is_err())
    }

    #[test]
    fn mul_2x2_f32() {
        let mat_a = matrix_f32!("{{1,1},{1,1}}", TOL).unwrap();
        let mat_b = matrix_f32!("{{2,2},{2,2}}", TOL).unwrap();
        let computed = mat_a.checked_mul(&mat_b).unwrap();
        let expected = matrix_f32!("{{4,4},{4,4}}", TOL).unwrap();

        pretty_assertions::assert_eq!(computed, expected)
    }

    #[test]
    fn mul_3x5x2_f32() {
        let mat_a = matrix_f32!("{{1,2,1,2,1}, {-1,2,-3,2,1}, {0,1,-3,2,1}}", TOL).unwrap();
        let mat_b = matrix_f32!("{{1,1}, {2,2}, {-1,-1}, {-2,-2}, {0,1}}", TOL).unwrap();
        let computed = mat_a.checked_mul(&mat_b).unwrap();
        let expected = matrix_f32!("{{0,1},{2,3},{1,2}}", TOL).unwrap();

        pretty_assertions::assert_eq!(computed, expected)
    }
}
