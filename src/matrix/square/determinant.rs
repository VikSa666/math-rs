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

    pub fn as_number<R: Ring>(&self) -> R {
        match self {
            Signature::Even => R::one(),
            Signature::Odd => -R::one(),
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

    /// TODO: Add other methods
    fn is_positive_definite(&self, tolerance: f32) -> bool {
        sylvester_criterion(self, tolerance)
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

    pub fn leading_principal_minor(&self, dimension: usize) -> Result<Self, MatrixError> {
        if dimension > self.dimension() {
            return Err(MatrixError::InvalidDimension(dimension));
        }
        let mut submatrix =
            SquareMatrix::new(dimension, vec![vec![R::zero(); dimension]; dimension]);
        for i in 0..dimension {
            for j in 0..dimension {
                submatrix.data_mut()[i][j] = self.data()[i][j].clone();
            }
        }
        Ok(submatrix)
    }
}

/// Sylvester's criterion for positive definiteness.
fn sylvester_criterion<R: Ring + PartialOrd>(matrix: &SquareMatrix<R>, tolerance: f32) -> bool {
    let dimension = matrix.dimension();
    let mut leading_principal_minors = Vec::new();
    for i in 1..dimension {
        let submatrix = matrix.leading_principal_minor(i).unwrap();
        leading_principal_minors
            .push(submatrix.determinant(DeterminantMethod::TriangleRule, tolerance));
    }
    let mut sign = Signature::Even;
    for minor in leading_principal_minors {
        if minor.unwrap() < R::zero() {
            sign.change();
        }
    }
    sign == Signature::Even
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

    if matrix_cloned.diagonal_is_zero(tolerance) {
        return Err(MatrixError::MatrixError(
            "Matrix has zero elements in diagonal".to_string(),
        ));
    }
    let dimension = matrix.dimension();
    let mut sign = Signature::Even;

    for k in 0..dimension - 1 {
        let mut diagonal_element = if k.checked_sub(1).is_none() {
            R::one()
        } else {
            matrix_cloned.data()[k - 1][k - 1].to_owned()
        };

        for i in k..dimension {
            if diagonal_element.is_zero(tolerance) {
                matrix_cloned.swap_rows(i, k - 1)?;
                println!("{matrix_cloned}");
                sign.change();
                diagonal_element = matrix_cloned.data()[k - 1][k - 1].to_owned();
            } else {
                break;
            }
        }

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

    Ok(
        matrix_cloned.data()[matrix.dimension() - 1][matrix.dimension() - 1].to_owned()
            * sign.as_number(),
    )
}

fn laplace_expansion<R: Ring + PartialOrd>(matrix: &SquareMatrix<R>) -> Result<R, MatrixError> {
    if matrix.dimension() == 1 {
        return Ok(matrix.data()[0][0].clone());
    }
    let mut determinant = R::zero();
    for column in 0..matrix.dimension() {
        let mut submatrix = SquareMatrix::new(
            matrix.dimension() - 1,
            vec![vec![R::zero(); matrix.dimension() - 1]; matrix.dimension() - 1],
        );
        for row in 1..matrix.dimension() {
            for column2 in 0..column {
                submatrix.data_mut()[row - 1][column2] = matrix.data()[row][column2].clone();
            }
            for column2 in column + 1..matrix.dimension() {
                submatrix.data_mut()[row - 1][column2 - 1] = matrix.data()[row][column2].clone();
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
    fn leading_principal_minors_should_be_ok() {
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
        let submatrix = matrix.unwrap().leading_principal_minor(2).unwrap();
        assert_eq!(
            submatrix,
            SquareMatrix::<Real>::try_from(vec![
                vec![Real::from_f32(1., TOL), Real::from_f32(2., TOL)],
                vec![Real::from_f32(1., TOL), Real::from_f32(-2., TOL)],
            ])
            .unwrap()
        );
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

    #[test]
    fn determinant_should_not_last_long() {
        let huge_matrix = SquareMatrix::from_fn(10, |i, j| {
            if (i as isize - j as isize).abs() < 3 {
                1
            } else {
                0
            }
        });

        println!("Matrix built!");

        let start = std::time::Instant::now();
        let determinant = huge_matrix
            .determinant(DeterminantMethod::LaplaceExpansion, 1E-10)
            .unwrap();
        let end = std::time::Instant::now();
        println!("time = {:?}", end - start);
        println!("determinant = {}", determinant)
    }
}
