# Mathrs

A mathematical library

For now, only matrices calculator is implemented.

## How to contribute

### Setting up the development environment

You will need Rust and `cargo` in order to develop this crate. Install _rustup_ in order to install them. Execute

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

You may need to configure your current shell. A message will appear if so, and you will be told to execute

```bash
source "$HOME/.cargo/env"
```

Later, to develop you will need the development server, which you can find in the `dev` folder. It is a Vue project, and to use it you will need:

1. Install `wasm-pack` with cargo: `cargo install wasm-pack`.
2. Compile and build the package. Navigate to `ffi/wasm` and execute `wasm-pack build --target web`.
3. Install `npm` and `nodejs`.
4. Install the dependencies with `cd dev` and then `npm install`.
5. Run the development server with `npm run serve`.

## What you can do

1. Calculate the sum, the substraction and the multiplication of matrices.
2. Calculate the reduced matrix using Gauss Triangulation.
3. Calculate the determinant matrix using both Gauss Triangulation and LU decomposition.
4. Calculate the inverse of a matrix using the Gauss-Jordan method.

## Usage in Rust

Every matrix can be created using a macro that will parse a string represenging the matrix in the following way: `{{a11, a12, a13, ...}. {a21, a22, a23, ...}, ...}`. As each matrix can contain different type of numbers, you have to specify which type of number will you get: `matrix_<number_type>!("{{a11, a12, ...}, {a21, a22, ...}, ...}")`.

### Examples

The call

```rust
let matrix = matrix_f32!("{{1.1, 2.2}, {2.1, 3.2}}");
println!("{matrix}")
```

will print

```asdf
+1.1000000 +2.2000000
+2.1000000 +3.2000000
```

## Usage in Javascript or Typescript

```ts
const performSum = async () => {
  await init();
  result.value = RMatrixF32.checked_sum(
    RMatrixF32.from_string(matA.value, 1e-6),
    RMatrixF32.from_string(matB.value, 1e-6)
  ).to_string();
};

const performSub = async () => {
  await init();
  result.value = RMatrixF32.checked_sub(
    RMatrixF32.from_string(matA.value, 1e-6),
    RMatrixF32.from_string(matB.value, 1e-6)
  ).to_string();
};

const performMul = async () => {
  await init();
  result.value = RMatrixF32.checked_mul(
    RMatrixF32.from_string(matA.value, 1e-6),
    RMatrixF32.from_string(matB.value, 1e-6)
  ).to_string();
};

const mat = ref("");
const result2 = ref("Nothing yet...");

const preformGaussReduction = async () => {
  await init();
  result2.value = RMatrixF32.from_string(mat.value, 1e-6)
    .gaussian_triangulation()
    .to_string();
};

const performGaussJordanDeterminant = async () => {
  await init();
  result2.value = RMatrixF32.from_string(mat.value, 1e-6)
    .determinant_using_gauss()
    .toString();
};

const performGaussJordanInverse = async () => {
  await init();
  result2.value = RMatrixF32.from_string(mat.value, 1e-6)
    .inverse_gauss_jordan()
    .to_string();
};
```
