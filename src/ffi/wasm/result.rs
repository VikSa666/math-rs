use wasm_bindgen::JsValue;

use crate::matrix::error::MatrixError;

impl From<MatrixError> for JsValue {
    fn from(value: MatrixError) -> Self {
        JsValue::from_str(value.to_string().as_str())
    }
}
