use std::fmt::Display;

use num::Num;

use super::Matrix;

impl<F: Num + Clone + Display> PartialEq for Matrix<F> {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}

impl<F: Num + Clone + Display> std::ops::Add for Matrix<F> {
    type Output = Result<Self, super::MatrixError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.rows() != rhs.rows() || self.columns() != rhs.columns() {
            return Err(super::MatrixError::InvalidNumberOfRows);
        }
        let mut result = self.clone();
        self.elements
            .iter()
            .enumerate()
            .for_each(|(row, row_elements)| {
                row_elements
                    .iter()
                    .enumerate()
                    .for_each(|(column, element)| {
                        let rhs_element = rhs.get(row, column).unwrap();
                        result.set(row, column, element.clone() + rhs_element.clone());
                    });
            });
        Ok(result)
    }
}

#[cfg(test)]
mod test {

    use crate::matrix::Matrix;

    #[test]
    fn add_i32() {
        let matrix = Matrix::<i32>::try_from(vec![vec![1, 2], vec![3, 4]]).unwrap();
        let matrix2 = Matrix::<i32>::try_from(vec![vec![1, 2], vec![3, 4]]).unwrap();
        let result = matrix + matrix2;
        assert_eq!(
            result.unwrap(),
            Matrix::<i32>::try_from(vec![vec![2, 4], vec![6, 8]]).unwrap()
        );
    }

    #[test]
    fn add_f64() {
        let matrix = Matrix::<f64>::try_from(vec![vec![1., 2.], vec![3., 4.]]).unwrap();
        let matrix2 = Matrix::<f64>::try_from(vec![vec![1., 2.], vec![3., 4.]]).unwrap();
        let result = matrix + matrix2;
        assert_eq!(
            result.unwrap(),
            Matrix::<f64>::try_from(vec![vec![2., 4.], vec![6., 8.]]).unwrap()
        );
    }
}
