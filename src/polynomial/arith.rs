use crate::{
    traits::{
        ArithmeticallyOperable, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Identity, Zero,
    },
    MathError,
};

use super::Polynomial;

impl Identity for Polynomial {
    fn id(_: usize, tolerance: f32) -> Self {
        Self {
            coefficients: vec![1.0],
            tolerance,
        }
    }
}

impl Zero for Polynomial {
    fn zero(_: usize, _: usize, tolerance: f32) -> Self {
        Self {
            coefficients: vec![0.0],
            tolerance,
        }
    }

    fn is_zero(&self) -> bool {
        self.coefficients.iter().all(|x| x.is_zero())
    }
}

impl CheckedAdd for Polynomial {
    type Output = Result<Self, MathError>;
    fn checked_add(&self, other: &Self) -> Result<Self, MathError> {
        let mut result = vec![0.0; self.coefficients.len().max(other.coefficients.len())];
        for i in 0..result.len() {
            result[i] = self.coefficients.get(i).unwrap_or(&0.0)
                + other.coefficients.get(i).unwrap_or(&0.0);
        }
        let mut result = Self {
            coefficients: result,
            tolerance: self.tolerance.max(other.tolerance),
        };
        result.cut_off();
        Ok(result)
    }
}

impl CheckedSub for Polynomial {
    type Output = Result<Self, MathError>;
    fn checked_sub(&self, other: &Self) -> Result<Self, MathError> {
        let mut result = vec![0.0; self.coefficients.len().max(other.coefficients.len())];
        for i in 0..result.len() {
            result[i] = self.coefficients.get(i).unwrap_or(&0.0)
                - other.coefficients.get(i).unwrap_or(&0.0);
        }
        let mut result = Self {
            coefficients: result,
            tolerance: self.tolerance.max(other.tolerance),
        };
        result.cut_off();
        Ok(result)
    }
}

impl CheckedMul for Polynomial {
    type Output = Result<Self, MathError>;
    fn checked_mul(&self, other: &Self) -> Result<Self, MathError> {
        let mut result = vec![0.0; self.coefficients.len() + other.coefficients.len() - 1];
        for i in 0..self.coefficients.len() {
            for j in 0..other.coefficients.len() {
                result[i + j] += self.coefficients[i] * other.coefficients[j];
            }
        }
        Ok(Self {
            coefficients: result,
            tolerance: self.tolerance.max(other.tolerance),
        })
    }
}

impl CheckedDiv for Polynomial {
    type Output = Result<(Self, Self), MathError>;
    /// Will return (q, r) being q the quotient and r the remainder
    fn checked_div(&self, other: &Self) -> Result<(Self, Self), MathError> {
        if other.is_zero() {
            return Err(MathError::DivisionByZero);
        }
        let mut quotient = Polynomial::new(
            vec![0.0; self.coefficients.len() - other.coefficients.len() + 1],
            self.tolerance.max(other.tolerance),
        );
        let mut remainder = self.clone();
        // At each step n = d*q+r
        while !remainder.is_zero() && remainder.degree() >= other.degree() {
            let t = Polynomial::new(
                vec![remainder.leading_term() / other.leading_term()],
                self.tolerance.max(other.tolerance),
            );
            quotient = quotient.checked_add(&t)?;
            remainder = remainder.checked_sub(&other.checked_mul(&t)?)?;
        }
        quotient.cut_off();
        Ok((quotient, remainder))
    }
}

impl ArithmeticallyOperable for Polynomial {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn polynomial_sum_easy() {
        let p1 = Polynomial::new(vec![1.0, 2.0, 3.0], 0.0);
        let p2 = Polynomial::new(vec![1.0, 2.0, 3.0], 0.0);
        let p3 = Polynomial::new(vec![2.0, 4.0, 6.0], 0.0);
        println!("{:?}", p1.checked_add(&p2).unwrap());
        pretty_assertions::assert_eq!(p1.checked_add(&p2).unwrap(), p3);
    }

    #[test]
    fn polynomial_sum_difficult() {
        let p1 = Polynomial::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], 0.0);
        let p2 = Polynomial::new(vec![1.0, 2.0, 3.0, 4.0, -5.0], 0.0);
        let p3 = Polynomial::new(vec![2.0, 4.0, 6.0, 8.0], 0.0);
        pretty_assertions::assert_eq!(p1.checked_add(&p2).unwrap(), p3);
    }

    #[test]
    fn polynomial_sub_easy() {
        let p1 = Polynomial::new(vec![1.0, 2.0, 3.0], 0.0);
        let p2 = Polynomial::new(vec![1.0, 2.0, 3.0], 0.0);
        let p3 = Polynomial::new(vec![0.0], 0.0);
        pretty_assertions::assert_eq!(p1.checked_sub(&p2).unwrap(), p3);
    }

    #[test]
    fn polynomial_sub_difficult() {
        let p1 = Polynomial::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], 0.0);
        let p2 = Polynomial::new(vec![1.0, 2.0, 3.0, 4.0, -5.0], 0.0);
        let p3 = Polynomial::new(vec![0.0, 0.0, 0.0, 0.0, 10.0], 0.0);
        pretty_assertions::assert_eq!(p1.checked_sub(&p2).unwrap(), p3);
    }

    #[test]
    fn polynomial_mul_easy() {
        let p1 = Polynomial::new(vec![1.0, 2.0, 3.0], 0.0);
        let p2 = Polynomial::new(vec![1.0, 2.0, 3.0], 0.0);
        let p3 = Polynomial::new(vec![1.0, 4.0, 10.0, 12.0, 9.0], 0.0);
        pretty_assertions::assert_eq!(p1.checked_mul(&p2).unwrap(), p3);
    }

    #[test]
    fn polynomial_mul_difficult() {
        let p1 = Polynomial::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], 0.0);
        let p2 = Polynomial::new(vec![1.0, 2.0, 3.0, 4.0, -5.0], 0.0);
        let p3 = Polynomial::new(
            vec![1.0, 4.0, 10.0, 20.0, 25.0, 24.0, 16.0, 0.0, -25.0],
            0.0,
        );
        pretty_assertions::assert_eq!(p1.checked_mul(&p2).unwrap(), p3);
    }

    #[test]
    fn polynomial_div_easy() {
        let p1 = Polynomial::new(vec![1.0, 2.0, 3.0], 0.0);
        let p2 = Polynomial::new(vec![1.0, 2.0, 3.0], 0.0);
        let expected_q = Polynomial::new(vec![1.0], 0.0);
        let expected_r = Polynomial::new(vec![0.0], 0.0);
        pretty_assertions::assert_eq!(p1.checked_div(&p2).unwrap().0, expected_q);
        pretty_assertions::assert_eq!(p1.checked_div(&p2).unwrap().1, expected_r);
    }

    #[test]
    fn polynomial_div_difficult() {
        let p1 = Polynomial::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], 0.0);
        let p2 = Polynomial::new(vec![1.0, 2.0, 3.0, 4.0, -5.0], 0.0);
        let expected_q = Polynomial::new(vec![-1.0], 0.0);
        let expected_r = Polynomial::new(vec![2.0, 4.0, 6.0, 8.0], 0.0);
        pretty_assertions::assert_eq!(p1.checked_div(&p2).unwrap().0, expected_q);
        pretty_assertions::assert_eq!(p1.checked_div(&p2).unwrap().1, expected_r);
    }
}
