#[derive(Debug)]
pub enum StructureError {
    ParseError(String),
}

impl From<std::num::ParseIntError> for StructureError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::ParseError(error.to_string())
    }
}

impl From<std::num::ParseFloatError> for StructureError {
    fn from(error: std::num::ParseFloatError) -> Self {
        Self::ParseError(error.to_string())
    }
}
