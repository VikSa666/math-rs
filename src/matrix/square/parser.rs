use std::str::FromStr;

use crate::structures::Ring;

use super::{MatrixError, SquareMatrix};

impl<R: Ring> SquareMatrix<R> {
    fn parse(input: &str) -> Result<Self, MatrixError> {
        let mut matrix = vec![];
        let processed_input = input.trim().split_whitespace().collect::<String>();
        let inner = processed_input
            .trim_start_matches('{')
            .trim_end_matches('}')
            .trim();
        for row_str in inner.split("},{") {
            let row = row_str
                .split(',')
                .map(|s| -> Result<R, MatrixError> {
                    R::from_str(s).map_err(|_| {
                        MatrixError::MatrixError(format!(
                            "Could not parse matrix due to parsing error",
                        ))
                    })
                })
                .collect::<Result<Vec<R>, MatrixError>>()?;
            matrix.push(row);
        }
        Self::try_from(matrix)
    }
}

impl<R: Ring> FromStr for SquareMatrix<R> {
    type Err = MatrixError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

/// Creates a matrix with [`Real`](crate::structures::reals::Real) elements from a string.
///
/// **Important**: You will need to import [`crate::structures::reals::Real`]
///
/// # Examples
///
/// ```ignore
/// import crate::structures::reals::Real;
///
/// let matrix = matrix_reals!("{{1,2,3},{1,2,3},{1,1,1}}").unwrap();
/// assert_eq!(matrix.elements, vec![
///    vec![Real::new(1.0), Real::new(2.0), Real::new(3.0)],
///   vec![Real::new(1.0), Real::new(2.0), Real::new(3.0)],
///   vec![Real::new(1.0), Real::new(1.0), Real::new(1.0)],
/// ]);
/// ```
///
/// # Errors
/// Will return error if some of the elements are not parsable as [`Real`](crate::structures::reals::Real).
/// Also if
#[macro_export]
macro_rules! square_matrix_reals {
    ($s:expr) => {
        Matrix::<Real>::from_str($s)
    };
}

#[macro_export]
macro_rules! square_matrix_integers {
    ($s:expr) => {
        Matrix::<Integer<i32>>::from_str($s)
    };
}

#[macro_export]
macro_rules! square_matrix_rationals {
    ($s:expr) => {
        Matrix::<Rational<i32>>::from_str($s)
    };
}

#[cfg(test)]
mod test {
    use crate::{
        equality::Equals,
        matrix::{generic::Matrix, square::SquareMatrix},
        structures::{integers::Integer, rationals::Rational, reals::Real, Ring},
    };
    use std::str::FromStr;

    #[test]
    fn parse_from_integers_should_not_fail() {
        struct TestCase<'a, R: Ring> {
            id: &'a str,
            input: &'a str,
            expected: SquareMatrix<R>,
        }
        vec![
            TestCase {
                id: "Square matrix 1x1",
                input: "{{1}}",
                expected: SquareMatrix {
                    dimension: 1,
                    data: vec![vec![Integer::new(1)]],
                },
            },
            TestCase {
                id: "Square matrix 2x2",
                input: "{{1,2},{1,2}}",
                expected: SquareMatrix {
                    dimension: 2,
                    data: vec![
                        vec![Integer::new(1), Integer::new(2)],
                        vec![Integer::new(1), Integer::new(2)],
                    ],
                },
            },
            TestCase {
                id: "Square matrix 3x3",
                input: "{{1,2,3},{1,2,3},{1,1,1}}",
                expected: SquareMatrix {
                    dimension: 3,
                    data: vec![
                        vec![Integer::new(1), Integer::new(2), Integer::new(3)],
                        vec![Integer::new(1), Integer::new(2), Integer::new(3)],
                        vec![Integer::new(1), Integer::new(1), Integer::new(1)],
                    ],
                },
            },
        ]
        .into_iter()
        .for_each(|test| {
            let matrix = SquareMatrix::<Integer<i32>>::parse(test.input);
            assert!(matrix.is_ok(), "Test case {} failed", test.id);
            assert!(
                matrix.unwrap().equals(&test.expected, 0.0001),
                "Test case {} failed",
                test.id
            );
        });
        let matrix_string = "{{1,2,3},{1,2,3},{1,1,1}}";
        let matrix = SquareMatrix::<Integer<i32>>::parse(matrix_string);
        assert!(matrix.is_ok());
        println!("{}", matrix.unwrap())
    }

    #[test]
    fn macro_calls_should_not_fail() {
        let matrix_integers = square_matrix_integers!("{{1,2,3},{1,2,3},{1,1,1}}").unwrap();
        println!("{}", matrix_integers);
        let matrix_rationals = square_matrix_rationals!("{{1,2,3},{1,2,3},{1,1,1}}").unwrap();
        println!("{}", matrix_rationals);
        let matrix_reals = square_matrix_reals!("{{1,2,3},{1,2,3},{1,1,1}}").unwrap();
        println!("{}", matrix_reals);
    }

    #[test]
    #[should_panic]
    fn macro_calls_should_fail() {
        let matrix_fail =
            square_matrix_integers!("{{1.1, 1.2, 1.3}, {1.1, 1.2, 1.3}, {1.1, 1.2, 1.3}}").unwrap();
        println!("{}", matrix_fail);
    }
}
