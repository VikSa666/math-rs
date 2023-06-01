use crate::{
    matrix::{Matrix, Parseable, Serializable},
    result::{MathError, Result},
};

use super::MatrixF32;

impl Parseable for MatrixF32 {
    fn parse(input: &str, tolerance: f32) -> Result<Self> {
        parse_matrix(input, tolerance)
    }
}

impl Serializable for MatrixF32 {
    fn serialize(&self) -> String {
        serialize_matrix(self)
    }
}

pub fn parse_matrix(input: &str, tolerance: f32) -> Result<MatrixF32> {
    let mut matrix = vec![];
    let processed_input = input.trim().split_whitespace().collect::<String>();
    let inner = processed_input
        .trim_start_matches('{')
        .trim_end_matches('}')
        .trim();
    for row_str in inner.split("},{") {
        let row = row_str
            .split(',')
            .map(|s| -> Result<f32> {
                s.parse().map_err(|_| {
                    MathError::MatrixError(format!("Could not parse matrix due to parsing error",))
                })
            })
            .collect::<Result<Vec<f32>>>()?;
        matrix.push(row);
    }
    MatrixF32::new(matrix, tolerance)
}

pub fn serialize_matrix(matrix: &MatrixF32) -> String {
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

    use super::{serialize_matrix, MatrixF32};

    const TOLERANCE: f32 = 1E-12;

    #[test]
    fn parse_2x2() {
        let matrix = MatrixF32::from_str("{{1,2},{2,3}}")
            .expect("Should have been able to parse this matrix");

        println!("{matrix}");
        pretty_assertions::assert_eq!(
            matrix,
            MatrixF32::new(vec![vec![1.0, 2.0], vec![2.0, 3.0]], TOLERANCE)
                .expect("Should've been able to built this matrix")
        )
    }

    #[test]
    fn parse_3x5() {
        let matrix = MatrixF32::from_str("{{1,2,3,4,5}, {5,4,3,2,1}, {0,0,0,0,0}}")
            .expect("Should have been able to parse this matrix");
        println!("{matrix}");
        pretty_assertions::assert_eq!(
            matrix,
            MatrixF32::new(
                vec![
                    vec![1.0, 2.0, 3.0, 4.0, 5.0],
                    vec![5.0, 4.0, 3.0, 2.0, 1.0],
                    vec![0.0, 0.0, 0.0, 0.0, 0.0]
                ],
                TOLERANCE
            )
            .expect("Should've been able to built this matrix")
        )
    }

    #[test]
    fn serialize_2x2() {
        let matrix = MatrixF32::new(vec![vec![1.1, 1.1], vec![1.1, 1.1]], TOLERANCE).unwrap();
        pretty_assertions::assert_str_eq!("{{1.1, 1.1}, {1.1, 1.1}}", serialize_matrix(&matrix))
    }
}
