mod result;

use crate::matrix::{matrix_usize, serialize_matrix, GenericMatrix, Matrix};
use wasm_bindgen::prelude::*;

/// Initialization function that automatically gets called when the module is loaded in WASM.
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    tracing_wasm::set_as_global_default();
    Ok(())
}

#[wasm_bindgen]
pub struct MatrixUsize {
    inner: GenericMatrix<usize>,
}

#[wasm_bindgen]
impl MatrixUsize {
    #[wasm_bindgen(constructor)]
    pub fn new(content: Vec<usize>, rows: usize, columns: usize) -> Result<MatrixUsize, JsValue> {
        if content.len() != rows * columns {
            return Err(JsValue::from_str(
                format!(
                    "Cannot build matrix of dimensions {rows}x{columns} with {} elements!",
                    content.len()
                )
                .as_str(),
            ));
        }
        let mut matrix: Vec<Vec<usize>> = Vec::with_capacity(rows);
        for i in 0..rows {
            let mut row: Vec<usize> = Vec::with_capacity(columns);
            for j in 0..columns {
                row.push(content[i * (columns - 1) + j])
            }
            matrix.push(row)
        }
        let inner = GenericMatrix::new(matrix);
        match inner {
            Ok(inner) => {
                tracing::info!("Matrix has been built correctly");
                Ok(MatrixUsize { inner })
            }
            Err(error) => Err(JsValue::from(error)),
        }
    }
    //     let inner = GenericMatrix::new(content);
    //     match inner {
    //         Ok(inner) => Ok(MatrixUsize { inner }),
    //         Err(error) => Err(JsValue::from(error)),
    //     }
    // }

    #[wasm_bindgen(getter)]
    pub fn rows(&self) -> usize {
        self.inner.rows()
    }

    #[wasm_bindgen(getter)]
    pub fn columns(&self) -> usize {
        self.inner.columns()
    }

    pub fn get(&self, row: usize, column: usize) -> Result<usize, JsValue> {
        let result = self.inner.get(row + 1, column + 1)?;
        Ok(result.clone())
    }

    pub fn sum(matrix_a: MatrixUsize, matrix_b: MatrixUsize) -> Result<MatrixUsize, JsValue> {
        let sum = (matrix_a.inner + matrix_b.inner)?;
        Ok(MatrixUsize { inner: sum })
    }

    pub fn from_string(input: &str) -> Result<MatrixUsize, JsValue> {
        Ok(MatrixUsize {
            inner: matrix_usize!(input)?,
        })
    }

    pub fn to_string(&self) -> String {
        serialize_matrix(&self.inner)
    }
}
