use crate::{matrix::error::MatrixError, structures::Ring};

use super::SquareMatrix;

mod bareiss;
pub mod ring;
use bareiss::bareiss_algorithm;
mod gaussian;
use gaussian::gaussian_elimination_determinant;
mod montante;
use montante::montante_algorithm;

pub enum DeterminantMethod {
    TriangleRule,
    BareissAlgorithm,
    LaplaceExpansion,
    GaussianElimination,
    Optimize,
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
    pub fn determinant(
        &self,
        determinant_method: DeterminantMethod,
        tolerance: f32,
    ) -> Result<R, MatrixError> {
        match determinant_method {
            DeterminantMethod::TriangleRule => triangle_rule(self),
            DeterminantMethod::BareissAlgorithm => bareiss_algorithm(self, tolerance),
            DeterminantMethod::LaplaceExpansion => montante_algorithm(self),
            DeterminantMethod::GaussianElimination => {
                gaussian_elimination_determinant(self, tolerance)
            }
            DeterminantMethod::Optimize => best_determinant_method(self, tolerance),
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
                submatrix[(i, j)] = self[(i, j)].clone();
            }
        }
        Ok(submatrix)
    }
}

fn best_determinant_method<R: Ring + PartialOrd>(
    matrix: &SquareMatrix<R>,
    tolerance: f32,
) -> Result<R, MatrixError> {
    if matrix.dimension() < 4 {
        return triangle_rule(matrix);
    }
    if matrix.dimension() < 10 {
        return bareiss_algorithm(matrix, tolerance);
    }
    montante_algorithm(matrix)
}

fn triangle_rule<R: Ring + PartialOrd>(matrix: &SquareMatrix<R>) -> Result<R, MatrixError> {
    match matrix.dimension() {
        1 => Ok(matrix[(0, 0)].to_owned()),
        2 => {
            let a = matrix[(0, 0)].to_owned();
            let b = matrix[(0, 1)].to_owned();
            let c = matrix[(1, 0)].to_owned();
            let d = matrix[(1, 1)].to_owned();
            Ok(a * d - b * c)
        }
        3 => {
            let a = matrix[(0, 0)].to_owned();
            let b = matrix[(0, 1)].to_owned();
            let c = matrix[(0, 2)].to_owned();
            let d = matrix[(1, 0)].to_owned();
            let e = matrix[(1, 1)].to_owned();
            let f = matrix[(1, 2)].to_owned();
            let g = matrix[(2, 0)].to_owned();
            let h = matrix[(2, 1)].to_owned();
            let i = matrix[(2, 2)].to_owned();
            Ok(a.to_owned() * e.to_owned() * i.to_owned()
                + b.to_owned() * f.to_owned() * g.to_owned()
                + c.to_owned() * d.to_owned() * h.to_owned()
                - c * e * g
                - b * d * i
                - a * f * h)
        }
        dim => Err(MatrixError::InvalidDimension(dim)),
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{matrix::square::SquareMatrix, num_types::FromF32, structures::reals::Real};

    const TOL: f32 = 1e-12;

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

    // #[test]
    // fn determinant_should_not_last_long() {
    //     let huge_matrix = SquareMatrix::from_fn(10, |i, j| {
    //         if (i as isize - j as isize).abs() < 3 {
    //             1
    //         } else {
    //             0
    //         }
    //     });

    //     println!("Matrix built!");

    //     let start = std::time::Instant::now();
    //     let determinant = huge_matrix
    //         .determinant(DeterminantMethod::LaplaceExpansion, 1E-10)
    //         .unwrap();
    //     let end = std::time::Instant::now();
    //     println!("time = {:?}", end - start);
    //     println!("determinant = {}", determinant)
    // }
}
