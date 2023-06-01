//! This crate just works for displaying matrices in a "beautiful way". Hence, it is just for debugging

use std::fmt::Display;

use crate::matrix::Matrix;

use super::MatrixF32;

impl Display for MatrixF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut prec = self.tolerance();
        let mut decimal_points: usize = 0;
        while prec.abs() < 1.0 {
            prec *= 10.0;
            decimal_points += 1;
        }
        Ok(for i in 0..self.rows() {
            for j in 0..self.columns() {
                write!(f, "{:+.decimal_points$} ", self.get(i, j)?)?
            }
            write!(f, "\n")?
        })
    }
}

#[cfg(test)]
mod test {
    use crate::matrix::float::MatrixF32;
    const TOLERANCE: f32 = 1e-12;

    #[test]
    #[ignore]
    fn debug() {
        let matrix = vec![
            vec![11111.1, 2.2, 3.3],
            vec![4.4, 5.5, 6.6],
            vec![7.7, 8.8, 9.9],
        ];
        let matrix = MatrixF32::new(matrix, TOLERANCE).unwrap();
        println!("{matrix}")
    }

    #[test]
    fn print_float() {
        let matrix = MatrixF32::new(
            vec![
                vec![1.1, 2.2, 3.3],
                vec![4.4, 5.5, 6.6],
                vec![7.7, 8.8, 9.9],
            ],
            1e-4,
        )
        .unwrap();
        let expected = "+1.10000 +2.20000 +3.30000 
+4.40000 +5.50000 +6.60000 
+7.70000 +8.80000 +9.90000 
";
        pretty_assertions::assert_eq!(matrix.to_string(), expected)
    }
}
