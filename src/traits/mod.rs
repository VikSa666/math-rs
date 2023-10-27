pub trait Abs {
    type Output;
    fn abs_value(&self) -> Self::Output;
}

macro_rules! impl_abs {
    ($($t:ty)*) => ($(
        impl Abs for $t {
            type Output = $t;
            fn abs_value(&self) -> Self::Output {
                self.abs()
            }
        }
    )*)
}

impl_abs!(i8 i16 i32 i64 i128 isize f32 f64);
