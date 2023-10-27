use crate::{
    matrix::{square::SquareMatrix, AsMatrix, MatrixError},
    structures::Ring,
};

use super::Signature;

/// Determinant of an NxN matrix using the [Bareiss algorithm](https://en.wikipedia.org/wiki/Bareiss_algorithm).
pub(super) fn bareiss_algorithm<R: Ring + PartialOrd>(
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

#[cfg(test)]
mod tests {
    use crate::{
        matrix::square::{determinant::bareiss::bareiss_algorithm, SquareMatrix},
        num_types::FromF32,
        structures::reals::Real,
    };

    const TOL: f32 = 1e-12;
    #[test]
    fn bareiss_algorithm_should_not_fail() {
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
        ])
        .unwrap();
        assert_eq!(
            bareiss_algorithm(&matrix, TOL),
            Ok(Real::from_f32(14., TOL))
        );
    }
}
