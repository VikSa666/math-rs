mod result;

use wasm_bindgen::prelude::*;

use crate::{
    matrix::{Invertible, Matrix, MatrixF32, Parseable, Serializable},
    matrix_f32,
    traits::{CheckedAdd, CheckedMul, CheckedSub},
};

// TODO: change this and make it a passable parameter
const TOLERANCE: f32 = 1e-10;

/// Initialization function that automatically gets called when the module is loaded in WASM.
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    tracing_wasm::set_as_global_default();
    Ok(())
}

#[wasm_bindgen]
pub struct RMatrixF32 {
    inner: MatrixF32,
}

#[wasm_bindgen]
impl RMatrixF32 {
    #[wasm_bindgen(constructor)]
    pub fn new(content: Vec<f32>, rows: usize, columns: usize) -> Result<RMatrixF32, JsValue> {
        if content.len() != rows * columns {
            return Err(JsValue::from_str(
                format!(
                    "Cannot build matrix of dimensions {rows}x{columns} with {} elements!",
                    content.len()
                )
                .as_str(),
            ));
        }
        let mut matrix: Vec<Vec<f32>> = Vec::with_capacity(rows);
        for i in 0..rows {
            let mut row: Vec<f32> = Vec::with_capacity(columns);
            for j in 0..columns {
                row.push(content[i * (columns - 1) + j])
            }
            matrix.push(row)
        }
        let inner = MatrixF32::new(matrix, TOLERANCE);
        match inner {
            Ok(inner) => {
                tracing::info!("Matrix has been built correctly");
                Ok(RMatrixF32 { inner })
            }
            Err(error) => Err(JsValue::from(error)),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn rows(&self) -> usize {
        self.inner.rows()
    }

    #[wasm_bindgen(getter)]
    pub fn columns(&self) -> usize {
        self.inner.columns()
    }

    pub fn get(&self, row: usize, column: usize) -> Result<f32, JsValue> {
        let result = self.inner.get(row + 1, column + 1)?;
        Ok(result.clone())
    }

    pub fn checked_sum(matrix_a: RMatrixF32, matrix_b: RMatrixF32) -> Result<RMatrixF32, JsValue> {
        let sum = matrix_a.inner.checked_add(&matrix_b.inner)?;
        Ok(RMatrixF32 { inner: sum })
    }

    pub fn checked_sub(matrix_a: RMatrixF32, matrix_b: RMatrixF32) -> Result<RMatrixF32, JsValue> {
        let sub = matrix_a.inner.checked_sub(&matrix_b.inner)?;
        Ok(RMatrixF32 { inner: sub })
    }

    pub fn checked_mul(matrix_a: RMatrixF32, matrix_b: RMatrixF32) -> Result<RMatrixF32, JsValue> {
        let mul = matrix_a.inner.checked_mul(&matrix_b.inner)?;
        Ok(RMatrixF32 { inner: mul })
    }

    pub fn from_string(input: &str, tolerance: f32) -> Result<RMatrixF32, JsValue> {
        Ok(RMatrixF32 {
            inner: matrix_f32!(input, tolerance)?,
        })
    }

    pub fn to_string(&self) -> String {
        self.inner.serialize()
    }

    pub fn gaussian_triangulation(&self) -> Result<RMatrixF32, JsValue> {
        let result = self.inner.gaussian_triangulation()?;
        Ok(RMatrixF32 { inner: result })
    }

    pub fn determinant_using_gauss(&self) -> Result<f32, JsValue> {
        let result = self
            .inner
            .determinant_using_gauss()
            .ok_or("Matrix is not square!")?;
        Ok(result)
    }

    pub fn determinant_using_lu(&self) -> Result<f32, JsValue> {
        let result = self
            .inner
            .determinant_using_lu()
            .ok_or("Matrix is not square!")?;
        Ok(result)
    }

    pub fn inverse_gauss_jordan(&self) -> Result<RMatrixF32, JsValue> {
        let result = self.inner.inverse_gauss_jordan()?;
        Ok(RMatrixF32 { inner: result })
    }
}
