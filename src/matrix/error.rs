#[derive(Debug, PartialEq, Eq)]
pub enum MatrixError {
    InvalidNumberOfRows,
    InvalidNumberOfColumns,
    MatrixError(String),
    ParseError(String),
}

impl std::fmt::Display for MatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatrixError::InvalidNumberOfRows => write!(f, "Invalid number of rows"),
            MatrixError::InvalidNumberOfColumns => write!(f, "Invalid number of columns"),
            MatrixError::MatrixError(e) => write!(f, "Matrix error: {}", e),
            MatrixError::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}
