use crate::structures::Ring;

use super::{generic::Matrix, AsMatrix};

impl<R: Ring + PartialOrd> std::fmt::Display for Matrix<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for row in self.data().iter() {
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

    use crate::{
        matrix::generic::Matrix,
        structures::{integers::Integer, rationals::Rational},
    };
    #[test]
    fn display_i32() {
        let matrix = Matrix::<i32>::try_from(vec![vec![1, 2], vec![3, 4]]).unwrap();
        println!("{}", matrix);
        pretty_assertions::assert_eq!(format!("{}", matrix), "1 2 \n3 4 \n");
    }

    #[test]
    fn display_integer() {
        let matrix = Matrix::<Integer<i32>>::try_from(vec![
            vec![Integer::new(1), Integer::new(2)],
            vec![Integer::new(3), Integer::new(4)],
        ])
        .unwrap();
        println!("{}", matrix);
        pretty_assertions::assert_eq!(format!("{}", matrix), "1 2 \n3 4 \n");
    }

    #[test]
    fn display_rational() {
        let matrix = Matrix::<Rational<i32>>::try_from(vec![
            vec![Rational::from(1), Rational::from(2)],
            vec![Rational::from(3), Rational::from(4)],
        ])
        .unwrap();
        println!("{}", matrix);
        pretty_assertions::assert_eq!(format!("{}", matrix), "1/1 2/1 \n3/1 4/1 \n");
    }
}
