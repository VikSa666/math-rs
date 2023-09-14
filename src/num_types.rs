/// Helper trait to convert any value to [`f32`] type.
pub trait AsF32 {
    /// This function is the equivalent to the primitive cast `as f32`.
    fn as_f32(&self) -> f32;
}

macro_rules! impl_as_f32 {
    ($($t:ty),*) => {
        $(impl AsF32 for $t {
            fn as_f32(&self) -> f32 {
                *self as f32
            }
        })*
    };
}

impl_as_f32!(isize, i8, i16, i32, i64, i128);

/// Helper trait to obtain any value from [`f32`] type.
///
/// As the [`f32`] type is not exact, a tolerance is required to
/// obtain the value. A custom implementation for types should be defined.
pub trait FromF32 {
    /// For primitive types, this function is the equivalent to the primitive cast `as`.
    fn from_f32(value: f32, tolerance: f32) -> Self;
}

macro_rules! impl_from_f32 {
    ($($t:ty),*) => {
        $(impl FromF32 for $t {
            fn from_f32(value: f32, _: f32) -> Self {
                value as $t
            }
        })*
    };
}

impl_from_f32!(isize, i8, i16, i32, i64, i128);
