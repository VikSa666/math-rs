use crate::{
    matrix::{square::SquareMatrix, AsMatrix, MatrixError},
    structures::Ring,
};

/// Gaussian elimination method for calculating the determinant of a matrix.
///
/// Source: <https://en.wikipedia.org/wiki/Gaussian_elimination#Computing_determinants>
pub(super) fn gaussian_elimination_determinant<R: Ring + PartialOrd>(
    matrix: &SquareMatrix<R>,
    tolerance: f32,
) -> Result<R, MatrixError> {
    let reduced = matrix.gaussian_elimination(tolerance)?;
    let mut determinant = R::one(0, 0);
    for i in 0..reduced.dimension() {
        determinant = determinant * reduced[(i, i)].to_owned();
    }
    Ok(determinant)
}

#[cfg(test)]
mod tests {
    use crate::{
        matrix::square::{determinant::gaussian::gaussian_elimination_determinant, SquareMatrix},
        num_types::FromF32,
        structures::reals::Real,
    };

    const TOL: f32 = 1e-10;

    #[test]
    fn gaussian_elimination_determinant_should_not_fail() {
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
            gaussian_elimination_determinant(&matrix, TOL),
            Ok(Real::from_f32(14., TOL))
        );
    }
}
