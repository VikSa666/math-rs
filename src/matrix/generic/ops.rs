use std::ops::{Add, Neg, Sub};

use crate::{
    equality::Equals,
    identities::Zero,
    matrix::{AsMatrix, MatrixError},
    structures::Ring,
};

use super::Matrix;

impl<R: Ring + PartialOrd> Equals for Matrix<R> {
    fn equals(&self, rhs: &Self, tolerance: f32) -> bool {
        if self.rows() != rhs.rows() || self.columns() != rhs.columns() {
            return false;
        }
        self.data
            .iter()
            .zip(rhs.data.iter())
            .all(|(row, other_row)| {
                row.iter()
                    .zip(other_row.iter())
                    .all(|(element, other_element)| element.equals(other_element, tolerance))
            })
    }
}

impl<R: Ring + PartialOrd> Add for Matrix<R> {
    type Output = Result<Self, super::MatrixError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.rows() != rhs.rows() || self.columns() != rhs.columns() {
            return Err(super::MatrixError::InvalidNumberOfRows);
        }
        let mut result = self.clone();
        for (row, row_elements) in self.data.iter().enumerate() {
            for (column, element) in row_elements.iter().enumerate() {
                let rhs_element = rhs.get(row, column)?;
                result.set(row, column, element.clone() + rhs_element.clone())?;
            }
        }
        Ok(result)
    }
}

impl<R: Ring + PartialOrd> Zero for Matrix<R> {
    fn zero() -> Self {
        Matrix::<R>::with_capacity(0, 0)
    }

    fn is_zero(&self, tolerance: f32) -> bool {
        self.data
            .iter()
            .all(|row| row.iter().all(|element| element.is_zero(tolerance)))
    }
}

impl<R: Ring + PartialOrd> Neg for Matrix<R> {
    type Output = Result<Self, MatrixError>;

    fn neg(self) -> Self::Output {
        let mut result = self.clone();
        for (row, row_elements) in self.data.iter().enumerate() {
            for (column, element) in row_elements.iter().enumerate() {
                result.set(row, column, -element.clone())?;
            }
        }
        Ok(result)
    }
}

impl<R: Ring + PartialOrd> Sub for Matrix<R> {
    type Output = Result<Self, MatrixError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.rows() != rhs.rows() || self.columns() != rhs.columns() {
            return Err(MatrixError::InvalidNumberOfRows);
        }
        let mut result = self.clone();
        for (row, row_elements) in self.data.iter().enumerate() {
            for (column, element) in row_elements.iter().enumerate() {
                let rhs_element = rhs.get(row, column)?;
                result.set(row, column, element.clone() - rhs_element.clone())?;
            }
        }
        Ok(result)
    }
}

impl<R: Ring + PartialOrd> std::ops::Mul for Matrix<R> {
    type Output = Result<Self, super::MatrixError>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.columns() != rhs.rows() {
            return Err(super::MatrixError::InvalidNumberOfRows);
        }
        let mut result = Matrix::<R>::with_capacity(self.rows(), rhs.columns());
        for row in 0..self.rows() {
            for column in 0..rhs.columns() {
                let mut sum = R::zero();
                for i in 0..self.columns() {
                    sum = sum + self.get(row, i)?.to_owned() * rhs.get(i, column)?.to_owned();
                }
                result.set(row, column, sum)?;
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod test {

    use crate::{
        equality::Equals,
        matrix::generic::Matrix,
        structures::{complex::Complex, integers::Integer, rationals::Rational, reals::Real},
    };

    #[test]
    fn equals_integers_i32() {
        let matrix_a = Matrix::<Integer<i32>>::try_from(vec![
            vec![Integer::<i32>::new(1), Integer::<i32>::new(2)],
            vec![Integer::<i32>::new(3), Integer::<i32>::new(4)],
        ]);
        let matrix_b = Matrix::<Integer<i32>>::try_from(vec![
            vec![Integer::<i32>::new(1), Integer::<i32>::new(2)],
            vec![Integer::<i32>::new(3), Integer::<i32>::new(4)],
        ]);

        assert!(matrix_a.unwrap().equals(&matrix_b.unwrap(), 0.));
    }

    #[test]
    fn equals_rational_i32() {
        let matrix_a = Matrix::<Rational<i32>>::try_from(vec![
            vec![
                Rational::<i32>::new(Integer::<i32>::new(1), Integer::<i32>::new(2)),
                Rational::<i32>::new(Integer::<i32>::new(2), Integer::<i32>::new(3)),
            ],
            vec![
                Rational::<i32>::new(Integer::<i32>::new(3), Integer::<i32>::new(4)),
                Rational::<i32>::new(Integer::<i32>::new(4), Integer::<i32>::new(5)),
            ],
        ]);

        let matrix_b = Matrix::<Rational<i32>>::try_from(vec![
            vec![
                Rational::<i32>::new(Integer::<i32>::new(1), Integer::<i32>::new(2)),
                Rational::<i32>::new(Integer::<i32>::new(2), Integer::<i32>::new(3)),
            ],
            vec![
                Rational::<i32>::new(Integer::<i32>::new(3), Integer::<i32>::new(4)),
                Rational::<i32>::new(Integer::<i32>::new(4), Integer::<i32>::new(5)),
            ],
        ]);

        assert!(matrix_a.unwrap().equals(&matrix_b.unwrap(), 0.));
    }

    #[test]
    fn equals_real() {
        let matrix_a = Matrix::<Real>::try_from(vec![
            vec![Real::new(1.), Real::new(2.)],
            vec![Real::new(3.), Real::new(4.)],
        ]);
        let matrix_b = Matrix::<Real>::try_from(vec![
            vec![Real::new(1.), Real::new(2.)],
            vec![Real::new(3.), Real::new(4.)],
        ]);
        assert!(matrix_a.unwrap().equals(&matrix_b.unwrap(), 1e-12));
    }

    #[test]
    fn equals_complex() {
        let matrix_a = Matrix::<Complex>::try_from(vec![
            vec![
                Complex::new(Real::new(1.), Real::new(2.)),
                Complex::new(Real::new(3.), Real::new(4.)),
            ],
            vec![
                Complex::new(Real::new(5.), Real::new(6.)),
                Complex::new(Real::new(7.), Real::new(8.)),
            ],
        ]);
        let matrix_b = Matrix::<Complex>::try_from(vec![
            vec![
                Complex::new(Real::new(1.), Real::new(2.)),
                Complex::new(Real::new(3.), Real::new(4.)),
            ],
            vec![
                Complex::new(Real::new(5.), Real::new(6.)),
                Complex::new(Real::new(7.), Real::new(8.)),
            ],
        ]);
        assert!(matrix_a.unwrap().equals(&matrix_b.unwrap(), 1e-12));
    }

    #[test]
    fn operate_integer_i32() {
        let matrix_a = Matrix::<Integer<i32>>::try_from(vec![
            vec![Integer::<i32>::new(1), Integer::<i32>::new(2)],
            vec![Integer::<i32>::new(3), Integer::<i32>::new(4)],
        ])
        .unwrap();
        let matrix_b = Matrix::<Integer<i32>>::try_from(vec![
            vec![Integer::<i32>::new(1), Integer::<i32>::new(2)],
            vec![Integer::<i32>::new(3), Integer::<i32>::new(4)],
        ])
        .unwrap();
        let sum = matrix_a.clone() + matrix_b.clone();
        let multiplication = matrix_a * matrix_b;

        assert!(sum.unwrap().equals(
            &Matrix::<Integer<i32>>::try_from(vec![
                vec![Integer::<i32>::new(2), Integer::<i32>::new(4)],
                vec![Integer::<i32>::new(6), Integer::<i32>::new(8)]
            ])
            .unwrap(),
            0.
        ),);

        assert!(multiplication.unwrap().equals(
            &Matrix::<Integer<i32>>::try_from(vec![
                vec![Integer::<i32>::new(7), Integer::<i32>::new(10)],
                vec![Integer::<i32>::new(15), Integer::<i32>::new(22)]
            ])
            .unwrap(),
            0.
        ),);
    }
}
