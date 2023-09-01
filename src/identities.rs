/// Representation of the identity for the [`Add`](std::ops::Add) operation.
pub trait Zero: Sized + Clone + std::ops::Add {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

/// Representation of the identity for the [`Mul`](std::ops::Mul) operation.
pub trait One: Sized + Clone + std::ops::Mul {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

macro_rules! impl_identities_for_primitive {
    ($($t:ty),*) => {
        $(impl Zero for $t {
            fn zero() -> Self {
                0 as $t
            }

            fn is_zero(&self) -> bool {
                *self == 0 as $t
            }
        }

        impl One for $t {
            fn one() -> Self {
                1 as $t
            }

            fn is_one(&self) -> bool {
                *self == 1 as $t
            }
        })*
    };
}

impl_identities_for_primitive!(isize, i8, i16, i32, i64, i128);
