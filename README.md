# Mathrs

A mathematical library

## Usage

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
