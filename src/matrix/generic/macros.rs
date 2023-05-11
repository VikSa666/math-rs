#[macro_export]
macro_rules! matrix_usize {
    ($expression:tt) => {
        GenericMatrix::<usize>::try_from($expression)
    };
}

pub use matrix_usize;
