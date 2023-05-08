mod result;

use crate::matrix::GenericMatrix;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MatrixUsize {
    inner: GenericMatrix<usize>,
}

#[wasm_bindgen]
impl MatrixUsize {
    #[wasm_bindgen(constructor)]
    pub fn new(content: Vec<usize>, rows: usize, columns: usize) -> Result<MatrixUsize, JsValue> {
        let inner = GenericMatrix::new(content, rows, columns);
        match inner {
            Ok(inner) => Ok(MatrixUsize { inner }),
            Err(error) => Err(JsValue::from(error)),
        }
    }

    pub fn sum(matrix_a: MatrixUsize, matrix_b: MatrixUsize) -> Result<MatrixUsize, JsValue> {
        Ok(MatrixUsize {
            inner: (matrix_a.inner + matrix_b.inner)?,
        })
    }
}
