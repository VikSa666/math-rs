use crate::result::{MathError, Result};

use crate::matrix::traits::{Identity, Matrix, Parseable};

#[derive(Debug, Clone)]
pub struct MatrixF32 {
    pub content: Vec<Vec<f32>>,
    tolerance: f32,
}

#[macro_export]
macro_rules! matrix_f32 {
    ($expression:tt, $tol:expr) => {
        MatrixF32::parse($expression, $tol)
    };
}
pub use matrix_f32;

impl Matrix for MatrixF32 {
    type T = f32;
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

    fn is_symmetric(&self) -> bool {
        if !self.is_square() {
            return false;
        }
        for i in 0..self.rows() {
            for j in 0..self.columns() {
                if self.get(i, j).unwrap() != self.get(j, i).unwrap() {
                    return false;
                }
            }
        }
        true
    }

    fn get(&self, row: usize, column: usize) -> Result<&Self::T> {
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

    fn get_mut(&mut self, row: usize, column: usize) -> Result<&mut Self::T> {
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

    fn set(&mut self, row: usize, column: usize, value: Self::T) -> Result<()> {
        *self.get_mut(row, column)? = value;
        Ok(())
    }

    fn swap_rows(&mut self, row1: usize, row2: usize) -> Result<()> {
        if row1 > self.rows() || row2 > self.rows() {
            return Err(MathError::MatrixError(format!(
                "Cannot swap rows {row1} and {row2} if the matrix has {} rows!",
                self.rows()
            )));
        }
        self.content.swap(row1, row2);
        Ok(())
    }

    fn transpose(&self) -> Self {
        let mut new_matrix = self.clone(); // Clone as we want a new matrix
        for i in 0..self.rows() {
            for j in 0..self.columns() {
                new_matrix
                    .set(j, i, self.get(i, j).unwrap().clone())
                    .unwrap();
            }
        }
        new_matrix
    }

    fn gaussian_triangulation(&self) -> Result<Self> {
        let mut new_matrix = self.clone();
        for i in 0..self.rows() {
            let mut max_row = i;
            for j in i + 1..self.rows() {
                if new_matrix.get(j, i).unwrap().abs() > new_matrix.get(max_row, i).unwrap().abs() {
                    max_row = j;
                }
            }
            new_matrix.swap_rows(i, max_row)?;
            if i < self.rows() - 1 && new_matrix.get(i, i).unwrap().abs() <= new_matrix.tolerance()
            {
                return Err(MathError::MatrixError(
                    "Cannot perform gaussian elimination on a matrix with zero pivot".to_string(),
                ));
            }
            for j in i + 1..self.rows() {
                let factor = new_matrix.get(j, i).unwrap() / new_matrix.get(i, i).unwrap();
                for k in i..self.columns() {
                    let new_value =
                        new_matrix.get(j, k).unwrap() - factor * new_matrix.get(i, k).unwrap();
                    new_matrix.set(j, k, new_value).unwrap();
                }
            }
        }
        Ok(new_matrix)
    }

    fn lu_decomposition(&self) -> Result<(Self, Self)> {
        if !self.is_square() {
            return Err(MathError::MatrixError(
                "Cannot perform LU decomposition on a non-square matrix".to_string(),
            ));
        }
        let mut lower = Self::id(self.rows(), self.tolerance());
        let mut upper = self.clone();
        for i in 0..self.rows() {
            for j in 0..self.columns() {
                if j < i {
                    let factor = upper.get(i, j).unwrap() / upper.get(j, j).unwrap();
                    lower.set(i, j, factor).unwrap();
                    for k in j..self.columns() {
                        let new_value =
                            upper.get(i, k).unwrap() - factor * upper.get(j, k).unwrap();
                        upper.set(i, k, new_value).unwrap();
                    }
                }
            }
        }
        Ok((lower, upper))
    }

    fn determinant_using_gauss(&self) -> Option<Self::T> {
        let gaussian_elimination_result = self.gaussian_triangulation().ok()?;
        let mut mult = Self::T::id(0, 0.0);
        for i in 0..gaussian_elimination_result.rows() {
            for j in 0..gaussian_elimination_result.columns() {
                if i == j {
                    let value = gaussian_elimination_result.get(i, j).unwrap();
                    mult = mult * *value;
                }
            }
        }
        Some(mult)
    }

    fn determinant_using_lu(&self) -> Option<Self::T> {
        let (_, upper) = self.lu_decomposition().ok()?;
        let mut mult = Self::T::id(0, 0.0);
        for i in 0..self.rows() {
            for j in 0..self.columns() {
                if i == j {
                    let upper_value = upper.get(i, j).unwrap();
                    mult = mult * *upper_value;
                }
            }
        }
        Some(mult)
    }

    fn cholesky_decomposition(&self) -> Result<Self> {
        todo!("To be done");
    }
}

impl MatrixF32 {
    pub fn new(content: Vec<Vec<f32>>, tolerance: f32) -> Result<Self> {
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
        Ok(Self { content, tolerance })
    }

    pub fn tolerance(&self) -> f32 {
        self.tolerance
    }
}

impl TryFrom<&str> for MatrixF32 {
    /// Performs the conversion from a string to the matrix, with default tolerance 1e-4
    fn try_from(value: &str) -> Result<Self> {
        MatrixF32::parse(value, 1e-4)
    }

    type Error = MathError;
}

#[cfg(test)]
mod test {
    use crate::matrix::traits::{Matrix, Parseable, CheckedAdd};

    use super::{matrix_f32, MatrixF32};
    use pretty_assertions;

    const TOLERANCE: f32 = 1e-10;

    #[test]
    fn get_1() {
        let matrix = MatrixF32::new(vec![vec![1f32, 2f32], vec![3f32, 4f32]], TOLERANCE).unwrap();
        let item = matrix.get(0, 1).unwrap();
        pretty_assertions::assert_eq!(item.clone(), 2f32)
    }

    #[test]
    fn get_2() {
        let matrix = MatrixF32::new(
            vec![
                vec![1.1, 2.2, 3.3],
                vec![4.4, 5.5, 6.6],
                vec![7.7, 8.8, 9.9],
            ],
            TOLERANCE,
        )
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
        let matrix_a = MatrixF32::new(vec![vec![1f32, 1f32], vec![2f32, 2f32]], TOLERANCE)
            .expect("Should've been able to built this matrix");
        let matrix_b = MatrixF32::new(vec![vec![1f32, 1f32], vec![2f32, 2f32]], TOLERANCE)
            .expect("Should've been able to built this matrix");
        pretty_assertions::assert_eq!(matrix_a, matrix_b)
    }

    #[test]
    fn create_matrix_from_macro_1() {
        let matrix = matrix_f32!("{{1,2},{3,4}}", TOLERANCE)
            .expect("Should have been able to build matrix from macro");
        println!("{matrix}");
        let other = matrix_f32!("{{1,2},{3,4}}", TOLERANCE).expect("asdf");
        pretty_assertions::assert_eq!(matrix, other)
    }

    #[test]
    fn sum_with_macro() {
        let result = (matrix_f32!("{{1,1},{1,1}}", TOLERANCE)
            .expect("asdf")
            .checked_add(&matrix_f32!("{{2,2},{2,2}}", TOLERANCE).expect("asdf")))
        .expect("asdf");
        println!("{result}")
    }

    #[test]
    fn gaussian_elimination_1() {
        let matrix = matrix_f32!("{{1,2,3},{4,5,6},{7,8,9}}", TOLERANCE).expect("asdf");
        let gauss = matrix.gaussian_triangulation().expect("asdf");
        pretty_assertions::assert_eq!(
            gauss,
            matrix_f32!(
                "{{+7.0000000000000, +8.0000000000000, +9.0000000000000},
                {+0.0000000000000, +0.8571428060532, +1.7142856121063 },
                {+0.0000000000000, +0.0000000000000, +0.0000000000000}}",
                TOLERANCE
            )
            .expect("asdf")
        );
    }

    #[test]
    fn gaussian_elimination_2() {
        let matrix =
            matrix_f32!("{{1,2,1,2,1},{-1,2,-3,2,1},{0,1,-3,2,1}}", TOLERANCE).expect("asdf");
        let gauss = matrix.gaussian_triangulation().expect("asdf");
        pretty_assertions::assert_eq!(
            gauss,
            matrix_f32!("{{1,2,1,2,1},{0,4,-2,4,2},{0,0,-2.5,1,0.5}}", TOLERANCE).expect("asdf")
        );
    }

    #[test]
    fn lu_decomposition_1() {
        let matrix = matrix_f32!("{{1,2,3},{4,5,6},{7,8,9}}", TOLERANCE).expect("asdf");
        let (l, u) = matrix.lu_decomposition().expect("asdf");
        pretty_assertions::assert_eq!(
            l,
            matrix_f32!("{{1,0,0},{4,1,0},{7,2,1}}", TOLERANCE).expect("asdf")
        );
        pretty_assertions::assert_eq!(
            u,
            matrix_f32!("{{1,2,3},{0,-3,-6},{0,0,0}}", TOLERANCE).expect("asdf")
        );
    }

    #[test]
    fn determinant_1() {
        let matrix = matrix_f32!("{{1,2,3},{4,5,6},{7,8,9}}", TOLERANCE).expect("asdf");
        let determinant = matrix.determinant_using_gauss().expect("asdf");
        pretty_assertions::assert_eq!(determinant, 0f32);
    }

    #[test]
    fn determinant_using_lu_1() {
        let matrix = matrix_f32!("{{1,2,3},{4,5,6},{7,8,1}}", TOLERANCE).expect("asdf");
        let (lower, upper) = matrix.lu_decomposition().expect("asdf");
        println!("{lower}");
        println!("{upper}");
        let determinant = matrix.determinant_using_lu().expect("asdf");
        pretty_assertions::assert_eq!(determinant, 24f32);
    }
}
