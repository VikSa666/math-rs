use crate::fields::Field;

use super::Polynomial;

impl<F: Field> std::fmt::Display for Polynomial<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for (i, coefficient) in self.coefficients.iter().enumerate() {
            if *coefficient == 0.0 {
                continue;
            }
            if !first {
                write!(f, " + ")?;
            }
            first = false;
            if *coefficient != 1.0 || i == 0 {
                write!(f, "{}", coefficient)?;
            }
            if i > 0 {
                write!(f, "x")?;
            }
            if i > 1 {
                write!(f, "^{}", i)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_display() {
        let polynomial = Polynomial::new(vec![1.0, 2.0, 3.0], 0.0001);
        pretty_assertions::assert_eq!(format!("{}", polynomial), "1 + 2x + 3x^2");
    }

    #[test]
    fn test_display_2() {
        let polynomial = Polynomial::new(vec![1.0, 0.0, 3.0], 0.0001);
        pretty_assertions::assert_eq!(format!("{}", polynomial), "1 + 3x^2");
    }
}
