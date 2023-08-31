pub mod integers;
pub mod rationals;

pub trait Group: std::ops::Add + Eq + Sized + Clone {
    fn identity() -> Self;
    fn inverse(&self) -> Self;
    fn op(&self, rhs: &Self) -> Self;
}

pub trait Ring: Group + std::ops::Mul {
    fn zero() -> Self;
    fn one() -> Self;
    fn is_zero(&self) -> bool;
    fn is_one(&self) -> bool;
    fn sum(&self, rhs: &Self) -> Self;
    fn mul(&self, rhs: &Self) -> Self;
    fn inverse_addition(&self) -> Self;
}
