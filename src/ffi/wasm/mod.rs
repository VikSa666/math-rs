mod result;

use wasm_bindgen::prelude::*;

use crate::{
    matrix::{generic::Matrix, AsMatrix},
    matrix_reals,
    structures::reals::Real,
};
use std::{fmt::Display, str::FromStr};

/// Initialization function that automatically gets called when the module is loaded in WASM.
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    tracing_wasm::set_as_global_default();
    Ok(())
}

#[wasm_bindgen]
pub struct MatrixReal {
    inner: Matrix<Real>,
}

impl Display for MatrixReal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[wasm_bindgen]
impl MatrixReal {
    #[wasm_bindgen(constructor)]
    pub fn new(content: Vec<f32>, rows: usize, columns: usize) -> Result<MatrixReal, JsValue> {
        if content.len() != rows * columns {
            return Err(JsValue::from_str(
                format!(
                    "Cannot build matrix of dimensions {rows}x{columns} with {} elements!",
                    content.len()
                )
                .as_str(),
            ));
        }
        let mut matrix: Vec<Vec<Real>> = Vec::with_capacity(rows);
        for i in 0..rows {
            let mut row: Vec<Real> = Vec::with_capacity(columns);
            for j in 0..columns {
                row.push(Real::new(content[i * (columns - 1) + j]))
            }
            matrix.push(row)
        }
        let inner = Matrix::<Real>::try_from(matrix);
        match inner {
            Ok(inner) => {
                tracing::info!("Matrix has been built correctly");
                Ok(MatrixReal { inner })
            }
            Err(error) => Err(JsValue::from_str(error.to_string().as_str())),
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
        Ok(result.value())
    }

    pub fn checked_sum(matrix_a: MatrixReal, matrix_b: MatrixReal) -> Result<MatrixReal, JsValue> {
        let sum = matrix_a.inner + matrix_b.inner;
        Ok(MatrixReal { inner: sum? })
    }

    pub fn checked_sub(matrix_a: MatrixReal, matrix_b: MatrixReal) -> Result<MatrixReal, JsValue> {
        let sub = matrix_a.inner - matrix_b.inner;
        Ok(MatrixReal { inner: sub? })
    }

    pub fn checked_mul(matrix_a: MatrixReal, matrix_b: MatrixReal) -> Result<MatrixReal, JsValue> {
        let mul = matrix_a.inner * matrix_b.inner;
        Ok(MatrixReal { inner: mul? })
    }

    pub fn from_string(input: &str) -> Result<MatrixReal, JsValue> {
        Ok(MatrixReal {
            inner: matrix_reals!(input)?,
        })
    }

    /// Method "to_string" but cannot name it like this because Clippy will complain:
    /// ```text
    /// warning: implementation of inherent method `to_string(&self) -> String` for type `ffi::wasm::MatrixReal`
    ///   --> src/ffi/wasm/mod.rs:91:5
    ///   |
    /// 91| /     pub fn to_string(&self) -> String {
    /// 92| |         self.inner.to_string()
    /// 93| |     }
    ///   | |_____^
    ///   |
    ///   = help: implement trait `Display` for type `ffi::wasm::MatrixReal` instead
    ///   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#inherent_to_string
    ///   = note: `#[warn(clippy::inherent_to_string)]` on by default
    /// ```
    pub fn convert_to_string(&self) -> String {
        self.inner.to_string()
    }

    pub fn gaussian_triangulation(&self) -> Result<MatrixReal, JsValue> {
        // let result = self.inner.gaussian_triangulation()?;
        // Ok(MatrixReal { inner: result })
        todo!()
    }

    pub fn determinant_using_gauss(&self) -> Result<f32, JsValue> {
        // let result = self
        //     .inner
        //     .determinant_using_gauss()
        //     .ok_or("Matrix is not square!")?;
        // Ok(result)
        todo!()
    }

    pub fn determinant_using_lu(&self) -> Result<f32, JsValue> {
        // let result = self
        //     .inner
        //     .determinant_using_lu()
        //     .ok_or("Matrix is not square!")?;
        // Ok(result)
        todo!()
    }

    pub fn inverse_gauss_jordan(&self) -> Result<MatrixReal, JsValue> {
        // let result = self.inner.inverse_gauss_jordan()?;
        // Ok(MatrixReal { inner: result })
        todo!()
    }
}
