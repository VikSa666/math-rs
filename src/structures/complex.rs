use std::{
    ops::{Add, Div, Mul, Neg, Rem, Sub},
    str::FromStr,
};

use crate::{
    equality::Equals,
    identities::{One, Zero},
    num_types::{AsF32, FromF32},
    traits::Abs,
};

use super::{errors::StructureError, reals::Real, Field, Group, Ring};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    re: Real,
    im: Real,
}

impl Complex {
    pub fn new(re: Real, im: Real) -> Self {
        Self { re, im }
    }

    pub fn conjugate(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn modulus(&self) -> Real {
        (self.re * self.re + self.im * self.im).sqrt()
    }
}

impl From<(f32, f32)> for Complex {
    fn from(value: (f32, f32)) -> Self {
        Self {
            re: Real::new(value.0),
            im: Real::new(value.1),
        }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Zero for Complex {
    fn zero() -> Self {
        Self {
            re: Real::zero(),
            im: Real::zero(),
        }
    }

    fn is_zero(&self, tolerance: f32) -> bool {
        self.re.is_zero(tolerance) && self.im.is_zero(tolerance)
    }
}

impl FromStr for Complex {
    type Err = StructureError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!("Parser for complex numbers is not yet implemented")
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.re.is_zero(1e-12) {
            if self.im.is_zero(1e-12) {
                write!(f, "0")
            } else {
                write!(f, "{:+}i", self.im)
            }
        } else if self.im.is_zero(1e-12) {
            write!(f, "{:+}", self.re)
        } else {
            write!(f, "{:+}{:+}i", self.re, self.im)
        }
    }
}

impl Equals for Complex {
    fn equals(&self, rhs: &Self, tolerance: f32) -> bool {
        self.re.equals(&rhs.re, tolerance) && self.im.equals(&rhs.im, tolerance)
    }
}

impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl FromF32 for Complex {
    fn from_f32(value: f32, _: f32) -> Self {
        Self {
            re: Real::new(value),
            im: Real::zero(),
        }
    }
}

impl AsF32 for Complex {
    /// The `as_f32` for complex numbers does not have sense. Hence, it should not be used.
    ///
    /// As a temporary workaround, it will return the modulus of the `Self`.
    fn as_f32(&self) -> f32 {
        self.modulus().as_f32()
    }
}

impl From<Real> for Complex {
    fn from(value: Real) -> Self {
        Self {
            re: value,
            im: Real::zero(),
        }
    }
}

impl Abs for Complex {
    type Output = Complex;

    fn abs_value(&self) -> Self::Output {
        Self::from(self.modulus())
    }
}

impl Group for Complex {
    fn identity() -> Self {
        Self::zero()
    }

    fn inverse(&self) -> Self {
        Self::neg(*self)
    }

    fn op(&self, rhs: &Self) -> Self {
        *self + *rhs
    }
}

impl One for Complex {
    fn one() -> Self {
        Self {
            re: Real::one(),
            im: Real::zero(),
        }
    }

    fn is_one(&self, tolerance: f32) -> bool {
        self.re.is_one(tolerance) && self.im.is_zero(tolerance)
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Rem for Complex {
    type Output = Self;

    fn rem(self, _: Self) -> Self::Output {
        todo!()
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let conj = rhs.conjugate();
        let norm = rhs.re * rhs.re + rhs.im * rhs.im;
        Self {
            re: (self * conj).re / norm,
            im: (self * conj).im / norm,
        }
    }
}

impl Ring for Complex {
    fn sum(&self, rhs: &Self) -> Self {
        *self + *rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        *self * *rhs
    }
}

impl Field for Complex {
    fn inverse_multiplication(&self) -> Self {
        let conj = self.conjugate();
        let norm = self.re * self.re + self.im * self.im;
        Self {
            re: conj.re / norm,
            im: conj.im / norm,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        equality::Equals,
        identities::One,
        structures::{complex::Complex, Field},
    };

    #[test]
    fn display_works_as_expected() {
        let z_1 = Complex::from((1., 4.));
        let z_2 = Complex::from((1., -4.));
        let z_3 = Complex::from((-1., 4.));
        let z_4 = Complex::from((-1., 0.));
        pretty_assertions::assert_eq!(z_1.to_string(), "+1+4i");
        pretty_assertions::assert_eq!(z_2.to_string(), "+1-4i");
        pretty_assertions::assert_eq!(z_3.to_string(), "-1+4i");
        pretty_assertions::assert_eq!(z_4.to_string(), "-1");
    }

    const TOL: f32 = 1e-12;

    #[test]
    fn sum_works_as_expected() {
        let z_1 = Complex::from((1., 4.));
        let z_2 = Complex::from((1., -4.));
        let z_3 = Complex::from((-1., 4.));
        let z_4 = Complex::from((-1., 0.));
        assert!((z_1 + z_2).equals(&Complex::from((2., 0.)), TOL));
        assert!((z_1 + z_3).equals(&Complex::from((0., 8.)), TOL));
        assert!((z_1 + z_4).equals(&Complex::from((0., 4.)), TOL));
        assert!((z_2 + z_3).equals(&Complex::from((0., 0.)), TOL));
        assert!((z_2 + z_4).equals(&Complex::from((0., -4.)), TOL));
        assert!((z_3 + z_4).equals(&Complex::from((-2., 4.)), TOL));
    }

    #[test]
    fn mul_works_as_expected() {
        let z_1 = Complex::from((1., 4.));
        let z_2 = Complex::from((1., -4.));
        let z_3 = Complex::from((-1., 4.));
        let z_4 = Complex::from((-1., 0.));
        assert!((z_1 * z_2).equals(&Complex::from((17., 0.)), TOL));
        assert!((z_1 * z_3).equals(&Complex::from((-17., 0.)), TOL));
        assert!((z_1 * z_4).equals(&Complex::from((-1., -4.)), TOL));
        assert!((z_2 * z_3).equals(&Complex::from((15., 8.)), TOL));
        assert!((z_2 * z_4).equals(&Complex::from((-1., 4.)), TOL));
        assert!((z_3 * z_4).equals(&Complex::from((1., -4.)), TOL));
    }

    #[test]
    fn norm_works_as_expected() {
        let z_1 = Complex::from((1., 4.));
        let z_2 = Complex::from((1., -4.));
        let z_3 = Complex::from((-1., 4.));
        let z_4 = Complex::from((-1., 0.));
        assert!((z_1.modulus().value() - 4.123105625617661).abs() < TOL);
        assert!((z_2.modulus().value() - 4.123105625617661).abs() < TOL);
        assert!((z_3.modulus().value() - 4.123105625617661).abs() < TOL);
        assert!((z_4.modulus().value() - 1.).abs() < TOL);
    }

    #[test]
    fn inverse_works_as_expected() {
        let z_1 = Complex::from((1., 4.));
        let z_2 = Complex::from((1., -4.));
        let z_3 = Complex::from((-1., 4.));
        let z_4 = Complex::from((-1., 0.));
        assert!((z_1.inverse_multiplication() * z_1).is_one(TOL));
        assert!((z_2.inverse_multiplication() * z_2).is_one(TOL));
        assert!((z_3.inverse_multiplication() * z_3).is_one(TOL));
        assert!((z_4.inverse_multiplication() * z_4).is_one(TOL));
    }
}
