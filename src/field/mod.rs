pub mod rationals;

use std::fmt::Debug;

use crate::traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Identity, Zero};

pub trait FieldElement:
    Clone + Debug + CheckedAdd + CheckedSub + CheckedDiv + CheckedMul + Zero + Identity
{
    fn eq_with_tolerance(&self, other: &Self, tolerance: f32) -> bool;
}

pub trait Field: Debug + Clone {
    type Element: FieldElement;
}
