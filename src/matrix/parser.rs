use std::fmt::Display;

use crate::{
    matrix::Matrix,
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

pub fn serialize_matrix<T>(matrix: &GenericMatrix<T>) -> String
where
    T: ArithmeticallyOperable<T> + Display,
{
    let mut result = String::new();
    let push_row = |res: &mut String, row_number: usize| {
        res.push('{');
        for j in 0..matrix.columns() - 1 {
            // TODO: Remove this unwrap
            res.push_str(matrix.get(row_number, j).unwrap().to_string().as_str());
            res.push_str(", ")
        }
        res.push_str(
            matrix
                .get(row_number, matrix.columns() - 1)
                .unwrap()
                .to_string()
                .as_str(),
        );
        res.push('}');
    };

    result.push('{');
    for i in 0..matrix.rows() - 1 {
        push_row(&mut result, i);
        result.push_str(", ");
    }
    push_row(&mut result, matrix.rows() - 1);

    result.push('}');
    result
}

#[cfg(test)]
mod test {

    use std::str::FromStr;

    use super::{serialize_matrix, GenericMatrix};

    #[test]
    fn parse_2x2() {
        let matrix = GenericMatrix::<usize>::from_str("{{1,2},{2,3}}")
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
        let matrix = GenericMatrix::<usize>::from_str("{{1,2,3,4,5}, {5,4,3,2,1}, {0,0,0,0,0}}")
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

    #[test]
    fn serialize_2x2() {
        let matrix = GenericMatrix::<f32>::new(vec![vec![1.1, 1.1], vec![1.1, 1.1]]).unwrap();
        pretty_assertions::assert_str_eq!("{{1.1, 1.1}, {1.1, 1.1}}", serialize_matrix(&matrix))
    }
}
