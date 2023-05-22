use crate::matrix::traits::{Identity, Invertible, Matrix};

use super::MatrixF32;

impl Invertible for MatrixF32 {
    fn inverse_gauss_jordan(&self) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let mut matrix = self.clone(); // I don't want to lose the original matrix
        let mut inverse = MatrixF32::id(self.rows(), self.tolerance());
        for i in 0..self.rows() {
            println!("Iteration {}", i);
            println!("{matrix}");
            println!("{inverse}");

            // Find the pivot
            let mut pivot_row = i;
            for j in i + 1..self.rows() {
                if matrix.get(j, i)?.abs() > matrix.get(pivot_row, i)?.abs() {
                    pivot_row = j;
                }
            }

            // Swap the rows
            if pivot_row != i {
                matrix.swap_rows(i, pivot_row)?;
                inverse.swap_rows(i, pivot_row)?;
            }

            // Divide the row by the pivot
            let pivot = matrix.get(i, i)?.clone();
            if pivot.abs() < self.tolerance() {
                return Err(crate::MathError::MatrixError(
                    "Matrix is not invertible".to_string(),
                ));
            }
            for j in 0..self.columns() {
                matrix.set(i, j, matrix.get(i, j)? / pivot)?;
                inverse.set(i, j, inverse.get(i, j)? / pivot)?;
            }

            // Subtract the row from all other rows
            for j in 0..self.rows() {
                if j != i {
                    let factor = matrix.get(j, i)?.to_owned();
                    for k in 0..self.columns() {
                        let new_value = matrix.get(j, k)? - factor * matrix.get(i, k)?;
                        matrix.set(j, k, new_value)?;
                        let new_value = inverse.get(j, k)? - factor * inverse.get(i, k)?;
                        inverse.set(j, k, new_value)?;
                    }
                }
            }

            println!("{matrix}");
            println!("{inverse}");
        }
        Ok(inverse)
    }

    fn inverse_montante(&self) -> crate::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn inverse_adjoint(&self) -> crate::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::matrix::traits::{Invertible, Parseable};
    use crate::matrix::MatrixF32;
    use crate::matrix_f32;

    const TOL: f32 = 1e-6;

    #[test]
    fn inverse_gauss_jordan_2x2_f32() {
        let mat_a = matrix_f32!("{{1,2},{3,4}}", TOL).unwrap();
        let mat_a_inv = mat_a.inverse_gauss_jordan().unwrap();
        let mat_a_inv_expected = matrix_f32!("{{-2,1},{1.5,-0.5}}", TOL).unwrap();
        pretty_assertions::assert_eq!(mat_a_inv, mat_a_inv_expected);
    }

    #[test]
    fn inverse_gauss_jordan_3x3_f32() {
        let mat_a = matrix_f32!("{{1,2,3},{0,1,4},{5,6,0}}", TOL).unwrap();
        let mat_a_inv = mat_a.inverse_gauss_jordan().unwrap();
        let mat_a_inv_expected = matrix_f32!("{{-24,18,5},{20,-15,-4},{-5,4,1}}", TOL).unwrap();
        pretty_assertions::assert_eq!(mat_a_inv, mat_a_inv_expected);
    }

    #[test]
    fn inverse_gauss_jordan_4x4_f32() {
        let mat_a = matrix_f32!("{{1,2,3,4},{0,1,4,5},{5,6,0,7},{8,9,10,0}}", TOL).unwrap();
        let mat_a_inv = mat_a.inverse_gauss_jordan().unwrap();
        let mat_a_inv_expected = matrix_f32!(
            "{{-0.5,0.5,0.5,-0.5},{-0.5,0.5,0.5,0.5},{0.5,-0.5,-0.5,0.5},{0.5,-0.5,-0.5,-0.5}}",
            TOL
        )
        .unwrap();
        pretty_assertions::assert_eq!(mat_a_inv, mat_a_inv_expected);
    }

    #[test]
    fn inverse_gauss_jordan_5x5_f32() {
        let mat_a = matrix_f32!(
            "{{1,2,3,4,5},{0,1,4,5,6},{5,6,0,7,8},{8,9,10,0,11},{12,13,14,15,0}}",
            TOL
        )
        .unwrap();
        let mat_a_inv = mat_a.inverse_gauss_jordan().unwrap();
        let mat_a_inv_expected = matrix_f32!("{{-0.5,0.5,0.5,-0.5,0.5},{-0.5,0.5,0.5,0.5,-0.5},{0.5,-0.5,-0.5,0.5,-0.5},{0.5,-0.5,-0.5,-0.5,0.5},{-0.5,0.5,0.5,0.5,0.5}}",TOL).unwrap();
        pretty_assertions::assert_eq!(mat_a_inv, mat_a_inv_expected);
    }
}
