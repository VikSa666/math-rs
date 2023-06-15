use std::str::FromStr;

use crate::{
    fields::Field,
    traits::{CheckedDiv, Zero},
    MathError, Result,
};

mod arith;
mod display;
mod scalar;
mod serde;
mod zeroes;

/// Representation of a polynomial by just saving its coefficients.
///
/// For example, the vector
///
/// ```asdf
/// [1.0, 2.0, 3.0]
/// ```
///
/// would represent the polynomial
///
/// ```tex
/// 1 + 2x + 3x^2
/// ```
#[derive(Debug, Clone)]
pub struct Polynomial<F: Field> {
    coefficients: Vec<F::Element>,
    tolerance: f32,
}

impl<F: Field> Polynomial<F> {
    pub fn new(coefficients: Vec<F::Element>, tolerance: f32) -> Self {
        let mut polynomial = Self {
            coefficients,
            tolerance,
        };
        polynomial.cut_off();
        polynomial
    }

    /// Returns the coefficients of the polynomial.
    #[inline]
    pub fn coefficients(&self) -> &[F::Element] {
        &self.coefficients
    }

    /// Returns the degree of the polynomial.
    #[inline]
    pub fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    /// Returns the leading term
    #[inline]
    pub fn leading_term(&self) -> F::Element {
        self.coefficients[self.degree()]
    }

    /// Normalizes the polynomial, i.e., divides all coefficients by the leading term.
    pub fn normalize(&mut self) {
        let leading_term = self.leading_term();
        for coefficient in self.coefficients.iter_mut() {
            *coefficient = leading_term.checked_div(coefficient);
        }
    }

    /// Cuts off the leading terms of the polynomial that are zero.
    ///
    /// Sometimes the following case could happen:
    ///
    /// ```tex
    /// a_0 + a_1x + a_2x^2 + ... + a_nx^n + 0x^(n+1) + 0x^(n+2) + ...
    /// ```
    ///
    /// and therefore, the zero terms to the right are cut off.
    pub fn cut_off(&mut self) {
        while self.leading_term().abs() <= self.tolerance.into() && self.coefficients.len() > 1 {
            self.coefficients.pop();
        }
    }

    /// Evaluates the polynomial at a given point, i.e., f(x).
    pub fn evaluate(&self, x: f64) -> f64 {
        let mut result = 0.0;
        for (i, coefficient) in self.coefficients.iter().enumerate() {
            result += coefficient * x.powi(i as i32);
        }
        result
    }

    /// Compute the derivative of the polynomial.
    pub fn differentiate(&self) -> Self {
        let mut coefficients = Vec::new();
        for (i, coefficient) in self.coefficients.iter().enumerate() {
            if i > 0 {
                coefficients.push(coefficient * (i as f64));
            }
        }
        Self {
            coefficients,
            ..*self
        }
    }

    /// Performs the least common multiple of two polynomials using euclidean division.
    ///
    /// Source: [Wikipedia](https://es.wikipedia.org/wiki/M%C3%A1ximo_com%C3%BAn_divisor_polin%C3%B3mico#MCD_mediante_c%C3%A1lculo_manual)
    pub fn lcd(&self, other: &Self) -> Result<Self> {
        // TODO: study how to remove these ugly clones
        let mut r_first = self.clone();
        let mut r_second = other.clone();
        while !r_second.is_zero() {
            let temp = r_second.clone();
            (_, r_second) = r_first.checked_div(&r_second)?;
            r_first = temp;
            println!("{:?}", r_second)
        }
        Ok(r_first)
    }
}

impl<F: Field> FromStr for Polynomial<F> {
    type Err = MathError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        todo!()
    }
}

impl<F: Field> PartialEq for Polynomial<F> {
    fn eq(&self, other: &Self) -> bool {
        if self.coefficients.len() != other.coefficients.len() {
            return false;
        }
        for i in 0..self.coefficients.len() {
            if (self.coefficients[i] - other.coefficients[i]).abs() > self.tolerance.into() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const TOLERANCE: f32 = 1e-10;
    #[test]
    fn test_cut_off() {
        let mut polynomial = Polynomial::new(vec![1.0, 2.0, 3.0, 0.0, 0.0], 0.0001);
        polynomial.cut_off();
        pretty_assertions::assert_eq!(polynomial, Polynomial::new(vec![1.0, 2.0, 3.0], 0.0001));
    }

    #[test]
    fn test_evaluate() {
        let polynomial = Polynomial::new(vec![1.0, 2.0, 3.0], 0.0001);
        pretty_assertions::assert_eq!(polynomial.evaluate(2.0), 17.0);
    }

    #[test]
    fn test_lcd() {
        let first = Polynomial::new(vec![-1.0, 0.0, 1.0], TOLERANCE);
        let second = Polynomial::new(vec![-1.0, 1.0], TOLERANCE);
        let computed_lcd = first.lcd(&second).unwrap();
        let expected_lcd = Polynomial::new(vec![-1.0, 1.0], TOLERANCE);
        pretty_assertions::assert_eq!(computed_lcd, expected_lcd);
    }

    #[test]
    fn test_lcd_2() {
        let first = Polynomial::new(vec![6.0, 7.0, 1.0], TOLERANCE);
        let second = Polynomial::new(vec![-6.0, -5.0, 1.0], TOLERANCE);
        let computed_lcd = first.lcd(&second).unwrap();
        let expected_lcd = Polynomial::new(vec![1.0, 1.0], TOLERANCE);
        pretty_assertions::assert_eq!(computed_lcd, expected_lcd);
    }
}
