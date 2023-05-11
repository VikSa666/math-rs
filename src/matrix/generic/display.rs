//! This crate just works for displaying matrices in a "beautiful way". Hence, it is just for debugging

use std::fmt::Display;

use super::{
    traits::{ArithmeticOperation, Matrix},
    GenericMatrix,
};

impl<T: ArithmeticOperation<T> + Display> Display for GenericMatrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for i in 0..self.rows() {
            for j in 0..self.columns() {
                write!(f, "{:+.12} ", self.get(i + 1, j + 1)?)?
            }
            write!(f, "\n")?
        })
    }
}

// impl<T: Add + Mul + Sub + Div + Display> From<&str> for Matrix<T> {
//     fn from(value: &str) -> Self {
//         let mut level = 0;
//         let mut matrix_content: Vec<T> = Vec::new();
//         value.chars().into_iter().for_each(|ch| match ch {
//             c if c.is_whitespace() => (),
//             c if c.is_numeric() => matrix_content.push(c.into()),
//             _ => todo!(),
//         });
//         todo!()
//     }
// }

#[cfg(test)]
mod test {
    use crate::matrix::GenericMatrix;

    #[test]
    #[ignore]
    fn debug() {
        let matrix = vec![
            vec![11111.1, 2.2, 3.3],
            vec![4.4, 5.5, 6.6],
            vec![7.7, 8.8, 9.9],
        ];
        let matrix = GenericMatrix::new(matrix).unwrap();
        println!("{matrix}")
    }

    #[test]
    fn print_float() {
        let matrix = GenericMatrix::new(vec![
            vec![1.1, 2.2, 3.3],
            vec![4.4, 5.5, 6.6],
            vec![7.7, 8.8, 9.9],
        ])
        .unwrap();
        let expected = "+1.100000000000 +2.200000000000 +3.300000000000 
+4.400000000000 +5.500000000000 +6.600000000000 
+7.700000000000 +8.800000000000 +9.900000000000 
";
        pretty_assertions::assert_eq!(matrix.to_string(), expected)
    }
}
