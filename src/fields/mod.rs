use std::fmt::Debug;

use crate::traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Identity, Zero};

pub trait FieldElement:
    Clone + Debug + CheckedAdd + CheckedSub + CheckedDiv + CheckedMul + Zero + Identity
{
}

impl FieldElement for f32 {}

pub trait Field: Clone {
    type Element: FieldElement;
}

#[derive(Debug, Clone)]
/// Represents the field of the rational numbers. For now, we will use
/// the [`f32`] type to represent the rational numbers.
pub struct Rationals;

impl Field for Rationals {
    type Element = f32;
}
