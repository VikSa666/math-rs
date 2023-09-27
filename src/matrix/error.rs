#[derive(Debug, PartialEq, Eq)]
pub enum MatrixError {
    InvalidNumberOfRows,
    InvalidNumberOfColumns,
    ElementNotFound(usize, usize),
    RowOutOfBounds(usize),
    ColumnOutOfBounds(usize),
    MatrixError(String),
    ParseError(String),
    NonSquareMatrix,
}

impl std::fmt::Display for MatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatrixError::InvalidNumberOfRows => write!(f, "Invalid number of rows"),
            MatrixError::InvalidNumberOfColumns => write!(f, "Invalid number of columns"),
            MatrixError::ElementNotFound(row, column) => {
                write!(f, "The element ({row}, {column}) was not found")
            }
            MatrixError::RowOutOfBounds(row) => write!(f, "The row {} is out of bounds", row),
            MatrixError::ColumnOutOfBounds(column) => {
                write!(f, "The column {} is out of bounds", column)
            }
            MatrixError::MatrixError(e) => write!(f, "Matrix error: {}", e),
            MatrixError::ParseError(e) => write!(f, "Parse error: {}", e),
            MatrixError::NonSquareMatrix => write!(f, "The matrix is not square"),
        }
    }
}
