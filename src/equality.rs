/// Substitute trait that implements the "==" operation with a tolerance.
///
/// This is implemented in order to have equality for real numbers (i.e., `f32` and `f64`), with
/// a given tolerance.
pub trait Equals {
    /// Returns wether `self` and `rhs` are equal, with a given tolerance.
    ///
    /// **Note**: The tolerance will be ignored for primitive `isize`, `i8`, `i16`, `i32`, `i64`, `i128` types.
    fn equals(&self, rhs: &Self, tolerance: f32) -> bool;
}

macro_rules! impl_equals_for_primitives {
    ($($t:ty),*) => {
        $(impl Equals for $t {
            fn equals(&self, rhs: &Self, _: f32) -> bool {
                self == rhs
            }
        })*
    };
}

impl_equals_for_primitives!(isize, i8, i16, i32, i64, i128);

impl Equals for f32 {
    fn equals(&self, rhs: &Self, tolerance: f32) -> bool {
        (self - rhs).abs() < tolerance
    }
}

impl Equals for f64 {
    fn equals(&self, rhs: &Self, tolerance: f32) -> bool {
        (self - rhs).abs() < tolerance as f64
    }
}
