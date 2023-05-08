use wasm_bindgen::JsValue;

use crate::result::MathError;

impl From<MathError> for JsValue {
    fn from(value: MathError) -> Self {
        match value {
            MathError::MatrixError(error) => JsValue::from_str(&error),
        }
    }
}
