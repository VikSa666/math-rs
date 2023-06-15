use crate::fields::Field;

use super::Polynomial;

fn newton_step<F: Field>(f: &Polynomial<F>, x: f64) -> f64 {
    x - f.evaluate(x) / f.differentiate().evaluate(x)
}

impl<F: Field> Polynomial<F> {
    /// Find the zeroes of the polynomial using Newton's method.
    ///
    /// TODO: Specify the field where this is performed (i.e., the complex numbers).
    fn find_real_zeroes_newton(&self, x0: f64, tolerance: f64, max_iterations: u128) -> Vec<f64> {
        let mut zeroes = Vec::new();
        let mut x = x0;
        for _ in 0..max_iterations {
            let x1 = newton_step(&self, x);
            if (x1 - x).abs() < tolerance {
                zeroes.push(x1);
                break;
            }
            println!("x1 - x = {}", (x1 - x).abs());
            x = x1;
        }
        zeroes
    }

    /// Execute the Ruffini's rule on the polynomial.
    fn ruffini(&self, x: f64) -> Polynomial<F> {
        let mut coefficients = Vec::new();
        let mut remainder = 0.0;
        for coefficient in self.coefficients.iter() {
            remainder = remainder * x + coefficient;
            coefficients.push(remainder);
        }
        Polynomial::new(coefficients, self.tolerance)
    }

    /// Given `f(x) = ax+b` returns the zero of the polynomial `f`, i.e., `-b/a`.
    fn zero_degree_1(&self) -> Option<f64> {
        if self.degree() != 1 {
            return None;
        }
        Some(-self.coefficients[0] / self.coefficients[1])
    }

    /// Returns the real roots of a 2-degree polynomial. It uses the common formula
    ///
    /// ```latex
    /// x_{1,2} = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
    /// ```
    fn quadratic_real(&self) -> Option<(f64, f64)> {
        if self.degree() != 2 {
            return None;
        }
        let a = self.coefficients[2];
        let b = self.coefficients[1];
        let c = self.coefficients[0];
        let delta = b * b - 4.0 * a * c;
        if delta < 0.0 {
            return None;
        }
        let x1 = (-b + delta.sqrt()) / (2.0 * a);
        let x2 = (-b - delta.sqrt()) / (2.0 * a);
        Some((x1, x2))
    }
}

#[cfg(test)]
mod test {

    use super::*;
    const TOLERANCE: f32 = 1e-10;
    const MAX_ITERATIONS: u128 = 1000;
    #[test]
    fn test_newton_step() {
        let polynomial = Polynomial::new(vec![1.0, 0.0, -1.0], TOLERANCE);
        pretty_assertions::assert_eq!(newton_step(&polynomial, 1.0), 1.0);
    }

    #[test]
    fn test_find_zeroes_newton() {
        let polynomial = Polynomial::new(vec![1.0, 0.0, -1.0], TOLERANCE);
        assert!(polynomial
            .find_real_zeroes_newton(1.0, TOLERANCE.into(), MAX_ITERATIONS)
            .iter()
            .all(|root| vec![1.0]
                .into_iter()
                .any(|expected| (root - expected).abs() < TOLERANCE.into())),);
    }

    #[test]
    fn test_find_zeroes_newton_multiple() {
        let polynomial = Polynomial::new(vec![1.0, 0.0, -1.0, 0.0, 1.0], TOLERANCE);
        assert!(polynomial
            .find_real_zeroes_newton(1.0, TOLERANCE.into(), MAX_ITERATIONS)
            .iter()
            .all(|root| vec![1.0, -1.0, 0.0]
                .into_iter()
                .any(|expected| (root - expected).abs() < TOLERANCE.into())),);
    }

    // TODO: These following tests are not pretty good because you cannot compare the vectors
    #[test]
    fn test_find_zeroes_newton_multiple_complex() {
        let polynomial = Polynomial::new(vec![1.0, 0.0, -1.0, 0.0, 1.0], TOLERANCE);
        assert!(polynomial
            .find_real_zeroes_newton(1.0, TOLERANCE.into(), MAX_ITERATIONS)
            .iter()
            .all(|root| vec![1.0, -1.0, 0.0]
                .into_iter()
                .any(|expected| (root - expected).abs() < TOLERANCE.into())),);
    }

    #[test]
    fn test_find_zeroes_newton_multiple_complex_2() {
        let polynomial = Polynomial::new(vec![5.643, 3.2, -1.5, 6.3, 4.1], TOLERANCE);
        assert!(polynomial
            .find_real_zeroes_newton(1.0, TOLERANCE.into(), MAX_ITERATIONS * 1000)
            .iter()
            .all(|root| vec![-0.861842175893627893]
                .into_iter()
                .any(|expected| (root - expected).abs() < TOLERANCE.into())),);
    }
}
