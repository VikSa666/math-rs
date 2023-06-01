use wasm_bindgen::JsValue;

use crate::result::MathError;

impl From<MathError> for JsValue {
    fn from(value: MathError) -> Self {
        match value {
            MathError::MatrixError(error) => JsValue::from_str(&error),
            MathError::PolynomialError(error) => JsValue::from_str(&error),
            MathError::DivisionByZero => JsValue::from_str("Division by zero"),
        }
    }
}
