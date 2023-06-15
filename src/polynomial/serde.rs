use crate::fields::Field;
use crate::traits::Parseable;
use crate::Result;

use super::Polynomial;

impl<F: Field> Parseable for Polynomial<F> {
    fn parse(s: &str, tolerance: f32) -> Result<Self> {
        let coefficients = s.trim();
        todo!()
    }
}
