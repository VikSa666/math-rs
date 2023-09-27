use crate::{
    matrix::{error::MatrixError, AsMatrix},
    structures::Ring,
};

use super::SquareMatrix;

pub enum DeterminantMethod {
    BareissAlgorithm,
    LaplaceExpansion,
}

impl<R: Ring + PartialOrd> SquareMatrix<R> {
    pub fn determinant(&self, determinant_method: DeterminantMethod) -> Result<R, MatrixError> {
        if self.dimension() == 1 {
            return self.get(0, 0).cloned();
        }
        if self.dimension() == 2 {
            let a = self.get(0, 0)?.clone();
            let b = self.get(0, 1)?.clone();
            let c = self.get(1, 0)?.clone();
            let d = self.get(1, 1)?.clone();
            return Ok(a * d - b * c);
        }
        if self.dimension() == 3 {
            let a = self.get(0, 0)?.clone();
            let b = self.get(0, 1)?.clone();
            let c = self.get(0, 2)?.clone();
            let d = self.get(1, 0)?.clone();
            let e = self.get(1, 1)?.clone();
            let f = self.get(1, 2)?.clone();
            let g = self.get(2, 0)?.clone();
            let h = self.get(2, 1)?.clone();
            let i = self.get(2, 2)?.clone();
            return Ok(a.clone() * e.clone() * i.clone()
                + b.clone() * f.clone() * g.clone()
                + c.clone() * d.clone() * h.clone()
                - c * e * g
                - b * d * i
                - a * f * h);
        }

        match determinant_method {
            DeterminantMethod::BareissAlgorithm => bareiss_algorithm(self),
            DeterminantMethod::LaplaceExpansion => laplace_expansion(self),
        }
    }
}

/// Determinant of an NxN matrix using the [Bareiss algorithm](https://en.wikipedia.org/wiki/Bareiss_algorithm).
fn bareiss_algorithm<R: Ring + PartialOrd>(matrix: &SquareMatrix<R>) -> Result<R, MatrixError> {
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
                    .unwrap()
                    .push(matrix.get(row, column2)?.clone());
            }
        }
        let sign = if column % 2 == 0 { R::one() } else { -R::one() };
        determinant =
            determinant + sign * matrix.data[0][column].clone() * bareiss_algorithm(&submatrix)?;
    }
    Ok(determinant)
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
                    .unwrap()
                    .push(matrix.get(row, column2)?.clone());
            }
        }
        let sign = if column % 2 == 0 { R::one() } else { -R::one() };
        determinant =
            determinant + sign * matrix.data[0][column].clone() * laplace_expansion(&submatrix)?;
    }
    Ok(determinant)
}

#[cfg(test)]
mod tests {
    use crate::matrix::square::SquareMatrix;

    #[test]
    fn bareiss_algorithm() {
        let matrix =
            SquareMatrix::<i32>::try_from(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 10]]);
    }
}
