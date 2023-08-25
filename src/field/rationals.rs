use super::{Field, FieldElement};

#[derive(Debug, Clone)]
/// Represents the field of the rational numbers. For now, we will use
/// the [`f32`] type to represent the rational numbers.
pub struct Rationals {
    tolerance: f32,
}

impl FieldElement for f32 {
    fn eq_with_tolerance(&self, other: &Self, tolerance: f32) -> bool {
        (self - other).abs() < tolerance
    }
}

impl Field for Rationals {
    type Element = f32;
}
