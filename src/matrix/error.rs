#[derive(Debug, PartialEq, Eq)]
pub enum MatrixError {
    InvalidNumberOfRows,
    InvalidNumberOfColumns,
    MatrixError(String),
}
