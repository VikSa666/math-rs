pub mod dimension;

use crate::structures::Ring;

use self::dimension::Dimension;

pub struct SquareMatrix<N, R>
where
    R: Ring,
    N: Dimension,
{
    dimension: N,
    data: Vec<Vec<R>>,
}

impl<N, R> SquareMatrix<N, R>
where
    R: Ring,
    N: Dimension,
{
    pub fn new(dimension: N, data: Vec<Vec<R>>) -> Self {
        Self { dimension, data }
    }

    pub fn dimension(&self) -> &N {
        &self.dimension
    }

    pub fn data(&self) -> &Vec<Vec<R>> {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::square::SquareMatrix;
    use crate::structures::integers::Integer;

    #[test]
    fn test_square_matrix() {
        let matrix = SquareMatrix::new(
            2,
            vec![
                vec![Integer::new(1), Integer::new(2)],
                vec![Integer::new(3), Integer::new(4)],
            ],
        );
        assert_eq!(matrix.dimension(), &2);
        assert_eq!(
            matrix.data(),
            &vec![
                vec![Integer::new(1), Integer::new(2)],
                vec![Integer::new(3), Integer::new(4)]
            ]
        );
    }

    #[test]
    fn test_square_matrix_dimension() {
        let matrix = SquareMatrix::new(
            2,
            vec![
                vec![Integer::new(1), Integer::new(2)],
                vec![Integer::new(3), Integer::new(4)],
            ],
        );
        assert_eq!(matrix.dimension(), &2);
    }

    #[test]
    fn test_square_matrix_data() {
        let matrix = SquareMatrix::new(
            2,
            vec![
                vec![Integer::new(1), Integer::new(2)],
                vec![Integer::new(3), Integer::new(4)],
            ],
        );
        assert_eq!(
            matrix.data(),
            &vec![
                vec![Integer::new(1), Integer::new(2)],
                vec![Integer::new(3), Integer::new(4)]
            ]
        );
    }
}
