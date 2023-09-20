use crate::num_types::AsF32;
use crate::structures::Ring;

use super::error::MatrixError;
use super::Matrix;

impl<R> Matrix<R>
where
    R: Ring + PartialOrd + AsF32,
{
    pub fn swap_rows(&mut self, row1: usize, row2: usize) {
        if row1 == row2 {
            return;
        }
        let data = self.data_mut();
        // TODO: Change this swap by a safe one
        data.swap(row1, row2)
    }

    fn gaussian_reduction(&self, tolerance: R) -> Result<Self, MatrixError> {
        let mut matrix = self.clone();
        let mut lead = 0;
        let rows = matrix.rows();
        let columns = matrix.columns();
        for r in 0..rows {
            if columns <= lead {
                break;
            }
            let mut i = r;
            while matrix
                .get(i, lead)
                .ok_or(MatrixError::ElementNotFound(i, lead))?
                .to_owned()
                < tolerance
            {
                i += 1;
                if rows == i {
                    i = r;
                    lead += 1;
                    if columns == lead {
                        break;
                    }
                }
            }
            matrix.swap_rows(i, r);
            if let Some(lead_value) = matrix.get(r, lead) {
                if !lead_value.is_zero(tolerance.as_f32()) {
                    for i in 0..rows {
                        if i != r {
                            let mut value = matrix
                                .get(i, lead)
                                .ok_or(MatrixError::ElementNotFound(i, lead))?
                                .to_owned()
                                / lead_value.to_owned();
                            for j in 0..columns {
                                let element = matrix.get_mut(i, j).unwrap();
                                *element = *element
                                    - value
                                        * matrix
                                            .get(r, j)
                                            .ok_or(MatrixError::ElementNotFound(r, j))?
                                            .to_owned();
                            }
                        }
                    }
                }
            }
            lead += 1;
        }
        Ok(matrix)
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;
    use crate::structures::integers::Integer;

    #[test]
    fn test_gaussian_reduction_2x2() {
        let matrix = Matrix::<Integer<i32>>::try_from(vec![
            vec![Integer::new(1), Integer::new(2)],
            vec![Integer::new(3), Integer::new(4)],
        ])
        .unwrap();
        let reduced = matrix.gaussian_reduction(0.0001).unwrap();
        println!("{reduced}");
        assert_eq!(
            reduced.data(),
            &vec![
                vec![Integer::new(1), Integer::new(2)],
                vec![Integer::new(0), Integer::new(-2)]
            ]
        );
    }
}
