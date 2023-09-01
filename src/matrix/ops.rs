use crate::structures::Ring;

use super::Matrix;

impl<R: Ring> PartialEq for Matrix<R> {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}

impl<R: Ring> std::ops::Add for Matrix<R> {
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

    use crate::{matrix::Matrix, structures::integers::Integer};

    #[test]
    fn add_i32() {
        let matrix = Matrix::<Integer<i32>>::try_from(vec![
            vec![Integer::<i32>::new(1), Integer::<i32>::new(2)],
            vec![Integer::<i32>::new(3), Integer::<i32>::new(4)],
        ])
        .unwrap();
        let matrix2 = Matrix::<Integer<i32>>::try_from(vec![
            vec![Integer::<i32>::new(1), Integer::<i32>::new(2)],
            vec![Integer::<i32>::new(3), Integer::<i32>::new(4)],
        ])
        .unwrap();
        let result = matrix + matrix2;
        assert_eq!(
            result.unwrap(),
            Matrix::<Integer<i32>>::try_from(vec![
                vec![Integer::<i32>::new(2), Integer::<i32>::new(4)],
                vec![Integer::<i32>::new(6), Integer::<i32>::new(8)]
            ])
            .unwrap()
        );
    }
}
