use std::fmt::Display;

use num::Num;

use super::Matrix;

impl<T: Num + Clone + Display> std::fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for row in self.elements.iter() {
            for element in row.iter() {
                result.push_str(&format!("{} ", element));
            }
            result.push_str("\n");
        }
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod test {
    use num::Complex;

    use crate::matrix::Matrix;
    #[test]
    fn display_i32() {
        let matrix = Matrix::<i32>::try_from(vec![vec![1, 2], vec![3, 4]]).unwrap();
        println!("{}", matrix);
        pretty_assertions::assert_eq!(format!("{}", matrix), "1 2 \n3 4 \n");
    }

    #[test]
    fn display_f64() {
        let matrix = Matrix::<f64>::try_from(vec![vec![1., 2.], vec![3., 4.]]).unwrap();
        println!("{}", matrix);
        pretty_assertions::assert_eq!(format!("{}", matrix), "1 2 \n3 4 \n");
    }

    #[test]
    fn display_complex() {
        let matrix = Matrix::<Complex<f32>>::try_from(vec![
            vec![Complex::<f32>::new(1., 1.), Complex::<f32>::new(1., 1.)],
            vec![Complex::<f32>::new(1., 1.), Complex::<f32>::new(1., 1.)],
        ])
        .unwrap();
        println!("{}", matrix);
        pretty_assertions::assert_eq!(format!("{}", matrix), "1+1i 1+1i \n1+1i 1+1i \n");
    }
}
