use crate::structures::Ring;

use super::Matrix;

impl<R: Ring> std::fmt::Display for Matrix<R> {
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

    use crate::matrix::Matrix;
    #[test]
    fn display_i32() {
        let matrix = Matrix::<i32>::try_from(vec![vec![1, 2], vec![3, 4]]).unwrap();
        println!("{}", matrix);
        pretty_assertions::assert_eq!(format!("{}", matrix), "1 2 \n3 4 \n");
    }
}
