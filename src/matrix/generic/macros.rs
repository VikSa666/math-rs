#[macro_export]
macro_rules! matrix_usize {
    ($expression:tt) => {
        GenericMatrix::<usize>::try_from($expression)
    };
}

pub use matrix_usize;

#[macro_export]
macro_rules! matrix_u8 {
    ($expression:tt) => {
        GenericMatrix::<u8>::try_from($expression)
    };
}

pub use matrix_u8;

#[macro_export]
macro_rules! matrix_u16 {
    ($expression:tt) => {
        GenericMatrix::<u16>::try_from($expression)
    };
}

pub use matrix_u16;

#[macro_export]
macro_rules! matrix_u32 {
    ($expression:tt) => {
        GenericMatrix::<u32>::try_from($expression)
    };
}

pub use matrix_u32;

#[macro_export]
macro_rules! matrix_u64 {
    ($expression:tt) => {
        GenericMatrix::<u64>::try_from($expression)
    };
}

pub use matrix_u64;

#[macro_export]
macro_rules! matrix_u128 {
    ($expression:tt) => {
        GenericMatrix::<u128>::try_from($expression)
    };
}

pub use matrix_u128;

#[macro_export]
macro_rules! matrix_isize {
    ($expression:tt) => {
        GenericMatrix::<isize>::try_from($expression)
    };
}

pub use matrix_isize;

#[macro_export]
macro_rules! matrix_i8 {
    ($expression:tt) => {
        GenericMatrix::<i8>::try_from($expression)
    };
}

pub use matrix_i8;

#[macro_export]
macro_rules! matrix_i16 {
    ($expression:tt) => {
        GenericMatrix::<i16>::try_from($expression)
    };
}

pub use matrix_i16;

#[macro_export]
macro_rules! matrix_i32 {
    ($expression:tt) => {
        GenericMatrix::<i32>::try_from($expression)
    };
}

pub use matrix_i32;

#[macro_export]
macro_rules! matrix_i64 {
    ($expression:tt) => {
        GenericMatrix::<i64>::try_from($expression)
    };
}

pub use matrix_i64;

#[macro_export]
macro_rules! matrix_i128 {
    ($expression:tt) => {
        GenericMatrix::<i128>::try_from($expression)
    };
}

pub use matrix_i128;

#[macro_export]
macro_rules! matrix_f32 {
    ($expression:tt) => {
        GenericMatrix::<f32>::try_from($expression)
    };
}

pub use matrix_f32;

#[macro_export]
macro_rules! matrix_f64 {
    ($expression:tt) => {
        GenericMatrix::<f64>::try_from($expression)
    };
}

pub use matrix_f64;

#[cfg(test)]
mod test {
    #[test]
    fn asdf() {
        let a: isize = 2 * (-3);
        println!("{:?}", a)
    }
}
