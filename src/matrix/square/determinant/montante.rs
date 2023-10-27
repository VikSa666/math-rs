use crate::{
    matrix::{square::SquareMatrix, AsMatrix, MatrixError},
    structures::Ring,
};

use super::Signature;

/// Implementation of the Montante's method, aka Laplace Expansion.
///
/// References:
/// - [Wikipedia](https://en.wikipedia.org/wiki/Laplace_expansion)
/// - [MathWorld](https://mathworld.wolfram.com/LaplaceExpansion.html)
/// - <https://informatika.stei.itb.ac.id/~rinaldi.munir/Matdis/2016-2017/Makalah2016/Makalah-Matdis-2016-051.pdf>
///
/// ## Errors
/// The function should never panic, but it may return an error if the matrix is not square.
///
/// ## Examples
/// ```txt
/// 1 2 3
/// 1 -2 0
/// 0 1 5
/// ```
/// The determinant of this matrix is -17.
///
/// ## Time complexity
/// The complexity of this algorithm is O(n!). Hence, it is not very good for big matrices.
pub(super) fn montante_algorithm<R: Ring + PartialOrd>(
    matrix: &SquareMatrix<R>,
) -> Result<R, MatrixError> {
    if matrix.dimension() == 1 {
        return Ok(matrix.data()[0][0].clone());
    }
    let mut determinant = R::zero();
    let mut sign = Signature::Even;
    for column in 0..matrix.dimension() {
        sign.change();
        let minor = matrix.minor(0, column)?;
        determinant = determinant
            + sign.as_number::<R>()
                * matrix.data()[0][column].clone()
                * montante_algorithm(&minor)?;
    }
    Ok(determinant)
}

#[cfg(test)]
mod tests {

    use crate::{
        matrix::square::{determinant::montante::montante_algorithm, SquareMatrix},
        num_types::FromF32,
        structures::integers::Integer,
    };

    const TOL: f32 = 1e-12;

    #[test]
    fn laplace_expansion_should_not_fail() {
        let matrix = SquareMatrix::<Integer<i32>>::try_from(vec![
            vec![Integer::from(1), Integer::from(2), Integer::from(3)],
            vec![Integer::from(1), Integer::from(-2), Integer::from(0)],
            vec![Integer::from(0), Integer::from(1), Integer::from(5)],
        ])
        .unwrap();
        pretty_assertions::assert_eq!(
            montante_algorithm(&matrix),
            Ok(Integer::from_f32(-17., TOL))
        );

        let matrix = SquareMatrix::<Integer<i32>>::from_fn(10, |i, j| {
            if i == j {
                Integer::from(1)
            } else {
                Integer::from(0)
            }
        });
        let start = std::time::Instant::now();
        let computed = montante_algorithm(&matrix);
        let time = start.elapsed().as_millis();
        println!("Montante's method took {} ms", time);
        assert!(time < 5000);
        pretty_assertions::assert_eq!(computed, Ok(Integer::from(-1)));
    }
}
