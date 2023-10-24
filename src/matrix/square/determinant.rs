use crate::{
    matrix::{error::MatrixError, AsMatrix},
    structures::Ring,
};

use super::SquareMatrix;

pub enum DeterminantMethod {
    TriangleRule,
    BareissAlgorithm,
    LaplaceExpansion,
}

/// Signature of a permutation.
///
/// TODO: This should not be here...
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Signature {
    Even = 1,
    Odd = -1,
}

impl Signature {
    pub fn new(sign: i32) -> Self {
        match sign {
            1 => Signature::Even,
            -1 => Signature::Odd,
            _ => panic!("Invalid sign"),
        }
    }

    pub fn change(&mut self) {
        match self {
            Signature::Even => *self = Signature::Odd,
            Signature::Odd => *self = Signature::Even,
        }
    }
}

impl<R: Ring + PartialOrd> SquareMatrix<R> {
    // TODO: Refactor as this is _n!_ in time complexity.
    fn maximize_diagonal(&mut self) -> Result<Signature, MatrixError> {
        let dimension = self.dimension();
        let mut sign = Signature::Even;
        for i in 0..dimension {
            let mut max_element_in_column = R::zero();
            for j in i..dimension {
                if self.get(j, i).unwrap().abs_value() > max_element_in_column.abs_value() {
                    max_element_in_column = self.get(j, i)?.clone();
                    self.swap_rows(i, j)?;
                    sign.change();
                }
            }
        }
        Ok(sign)
    }

    pub fn determinant(
        &self,
        determinant_method: DeterminantMethod,
        tolerance: f32,
    ) -> Result<R, MatrixError> {
        match determinant_method {
            DeterminantMethod::TriangleRule => triangle_rule(self),
            DeterminantMethod::BareissAlgorithm => bareiss_algorithm(self, tolerance),
            DeterminantMethod::LaplaceExpansion => laplace_expansion(self),
        }
    }
}

fn triangle_rule<R: Ring + PartialOrd>(matrix: &SquareMatrix<R>) -> Result<R, MatrixError> {
    match matrix.dimension() {
        1 => matrix.get(0, 0).cloned(),
        2 => {
            let a = matrix.get(0, 0)?.clone();
            let b = matrix.get(0, 1)?.clone();
            let c = matrix.get(1, 0)?.clone();
            let d = matrix.get(1, 1)?.clone();
            Ok(a * d - b * c)
        }
        3 => {
            let a = matrix.get(0, 0)?.clone();
            let b = matrix.get(0, 1)?.clone();
            let c = matrix.get(0, 2)?.clone();
            let d = matrix.get(1, 0)?.clone();
            let e = matrix.get(1, 1)?.clone();
            let f = matrix.get(1, 2)?.clone();
            let g = matrix.get(2, 0)?.clone();
            let h = matrix.get(2, 1)?.clone();
            let i = matrix.get(2, 2)?.clone();
            Ok(a.clone() * e.clone() * i.clone()
                + b.clone() * f.clone() * g.clone()
                + c.clone() * d.clone() * h.clone()
                - c * e * g
                - b * d * i
                - a * f * h)
        }
        dim => Err(MatrixError::InvalidDimension(dim)),
    }
}

/// Determinant of an NxN matrix using the [Bareiss algorithm](https://en.wikipedia.org/wiki/Bareiss_algorithm).
fn bareiss_algorithm<R: Ring + PartialOrd>(
    matrix: &SquareMatrix<R>,
    tolerance: f32,
) -> Result<R, MatrixError> {
    let mut matrix_cloned = matrix.clone();
    let sign = matrix_cloned.maximize_diagonal()?;
    if matrix_cloned.diagonal_is_zero(tolerance) {
        return Err(MatrixError::MatrixError(
            "Matrix has zero elements in diagonal".to_string(),
        ));
    }
    let dimension = matrix.dimension();

    for k in 0..dimension - 1 {
        let diagonal_element = if k.checked_sub(1).is_none() {
            R::one()
        } else {
            matrix_cloned.data()[k - 1][k - 1].to_owned()
        };
        for i in k + 1..dimension {
            for j in k + 1..dimension {
                let element = ((matrix_cloned.data()[i][j].to_owned()
                    * matrix_cloned.data()[k][k].to_owned())
                    - (matrix_cloned.data()[i][k].to_owned()
                        * matrix_cloned.data()[k][j].to_owned()))
                    / diagonal_element.clone();
                matrix_cloned.data_mut()[i][j] = element;
            }
        }
    }
    let sign = match sign {
        Signature::Even => R::one(),
        Signature::Odd => -R::one(),
    };

    Ok(matrix_cloned.data()[matrix.dimension() - 1][matrix.dimension() - 1].to_owned() * sign)
}

fn laplace_expansion<R: Ring + PartialOrd>(matrix: &SquareMatrix<R>) -> Result<R, MatrixError> {
    let mut determinant = R::zero();
    for column in 0..matrix.dimension() {
        let mut submatrix =
            SquareMatrix::with_capacity(matrix.dimension() - 1, matrix.dimension() - 1);
        for row in 1..matrix.dimension() {
            for column2 in 0..matrix.dimension() {
                if column2 == column {
                    continue;
                }
                submatrix
                    .data_mut()
                    .get_mut(row - 1)
                    .ok_or(MatrixError::RowOutOfBounds(row - 1))?
                    .push(matrix.get(row, column2)?.clone());
            }
        }
        let sign = if column % 2 == 0 { R::one() } else { -R::one() };
        determinant =
            determinant + sign * matrix.data()[0][column].clone() * laplace_expansion(&submatrix)?;
    }
    Ok(determinant)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        matrix::square::{
            determinant::{DeterminantMethod, Signature},
            SquareMatrix,
        },
        num_types::FromF32,
        structures::reals::Real,
    };

    const TOL: f32 = 1e-12;

    #[test]
    fn maximize_diagonal_returns_expected_signature() {
        let matrix = SquareMatrix::<Real>::try_from(vec![
            vec![
                Real::from_f32(0., TOL),
                Real::from_f32(1., TOL),
                Real::from_f32(5., TOL),
            ],
            vec![
                Real::from_f32(1., TOL),
                Real::from_f32(1., TOL),
                Real::from_f32(3., TOL),
            ],
            vec![
                Real::from_f32(1., TOL),
                Real::from_f32(-2., TOL),
                Real::from_f32(0., TOL),
            ],
        ]);
        let sign = matrix.unwrap().maximize_diagonal().unwrap();
        assert_eq!(sign, Signature::Even);
    }

    #[test]
    fn bareiss_algorithm() {
        let matrix = SquareMatrix::<Real>::try_from(vec![
            vec![
                Real::from_f32(1., TOL),
                Real::from_f32(2., TOL),
                Real::from_f32(3., TOL),
                Real::from_f32(4., TOL),
            ],
            vec![
                Real::from_f32(1., TOL),
                Real::from_f32(-2., TOL),
                Real::from_f32(0., TOL),
                Real::from_f32(1., TOL),
            ],
            vec![
                Real::from_f32(0., TOL),
                Real::from_f32(1., TOL),
                Real::from_f32(5., TOL),
                Real::from_f32(1., TOL),
            ],
            vec![
                Real::from_f32(1., TOL),
                Real::from_f32(-1., TOL),
                Real::from_f32(2., TOL),
                Real::from_f32(1., TOL),
            ],
        ]);
        assert_eq!(
            matrix
                .unwrap()
                .determinant(DeterminantMethod::BareissAlgorithm, TOL),
            Ok(Real::from_f32(14., TOL))
        );
    }

    #[test]
    fn laplace_expansion() {
        let matrix = SquareMatrix::<Real>::try_from(vec![
            vec![
                Real::from_f32(1., TOL),
                Real::from_f32(2., TOL),
                Real::from_f32(3., TOL),
            ],
            vec![
                Real::from_f32(1., TOL),
                Real::from_f32(-2., TOL),
                Real::from_f32(0., TOL),
            ],
            vec![
                Real::from_f32(0., TOL),
                Real::from_f32(1., TOL),
                Real::from_f32(5., TOL),
            ],
        ]);
        assert_eq!(
            matrix
                .unwrap()
                .determinant(DeterminantMethod::LaplaceExpansion, TOL),
            Ok(Real::from_f32(-17., TOL))
        );
    }
}
