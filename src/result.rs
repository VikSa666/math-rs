#[derive(Debug)]
pub enum MathError {
    MatrixError(String),
}

pub type Result<T> = core::result::Result<T, MathError>;

impl From<MathError> for std::fmt::Error {
    fn from(_: MathError) -> Self {
        std::fmt::Error
    }
}
