use std::str::FromStr;

use crate::MathError;

mod arith;
mod newton;
mod scalar;

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
pub struct Polynomial {
    coefficients: Vec<f64>,
    tolerance: f32,
}

impl Polynomial {
    pub fn new(coefficients: Vec<f64>, tolerance: f32) -> Self {
        Self {
            coefficients,
            tolerance,
        }
    }

    /// Returns the coefficients of the polynomial.
    #[inline]
    pub fn coefficients(&self) -> &[f64] {
        &self.coefficients
    }

    /// Returns the degree of the polynomial.
    #[inline]
    pub fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    /// Returns the leading term
    #[inline]
    pub fn leading_term(&self) -> f64 {
        self.coefficients[self.degree()]
    }

    /// Normalizes the polynomial, i.e., divides all coefficients by the leading term.
    pub fn normalize(&mut self) {
        let leading_term = self.leading_term();
        for coefficient in self.coefficients.iter_mut() {
            *coefficient /= leading_term;
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
}

impl FromStr for Polynomial {
    type Err = MathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl PartialEq for Polynomial {
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
    #[test]
    fn test_cut_off() {
        let mut polynomial = Polynomial::new(vec![1.0, 2.0, 3.0, 0.0, 0.0], 0.0001);
        polynomial.cut_off();
        assert_eq!(polynomial, Polynomial::new(vec![1.0, 2.0, 3.0], 0.0001));
    }
}
