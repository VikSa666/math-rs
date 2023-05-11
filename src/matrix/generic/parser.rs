use std::fmt::Display;

use crate::{
    matrix::traits::Parseable,
    result::{MathError, Result},
};

use super::{ArithmeticallyOperable, GenericMatrix};

pub fn parse_matrix<T: ArithmeticallyOperable<T> + Display>(
    input: &str,
) -> Result<GenericMatrix<T>> {
    let mut matrix = vec![];
    let processed_input = input.trim().split_whitespace().collect::<String>();
    let inner = processed_input
        .trim_start_matches('{')
        .trim_end_matches('}')
        .trim();
    for row_str in inner.split("},{") {
        let row = row_str
            .split(',')
            .map(|s| -> Result<T> {
                s.parse().map_err(|_| {
                    MathError::MatrixError(format!("Could not parse matrix due to parsing error",))
                })
            })
            .collect::<Result<Vec<T>>>()?;
        matrix.push(row);
    }
    GenericMatrix::new(matrix)
}

impl<T: ArithmeticallyOperable<T> + Display> Parseable<T> for GenericMatrix<T> {
    fn parse(expr: &str) -> Result<Self::Mat> {
        parse_matrix::<T>(expr)
    }

    type Mat = GenericMatrix<T>;
}

#[cfg(test)]
mod test {
    use crate::matrix::traits::Parseable;

    use super::GenericMatrix;

    #[test]
    fn parse_2x2() {
        let matrix = GenericMatrix::<usize>::parse("{{1,2},{2,3}}")
            .expect("Should have been able to parse this matrix");

        println!("{matrix}");
        pretty_assertions::assert_eq!(
            matrix,
            GenericMatrix::new(vec![vec![1, 2], vec![2, 3]])
                .expect("Should've been able to built this matrix")
        )
    }

    #[test]
    fn parse_3x5() {
        let matrix = GenericMatrix::<usize>::parse("{{1,2,3,4,5}, {5,4,3,2,1}, {0,0,0,0,0}}")
            .expect("Should have been able to parse this matrix");
        println!("{matrix}");
        pretty_assertions::assert_eq!(
            matrix,
            GenericMatrix::new(vec![
                vec![1, 2, 3, 4, 5],
                vec![5, 4, 3, 2, 1],
                vec![0, 0, 0, 0, 0]
            ])
            .expect("Should've been able to built this matrix")
        )
    }
}
