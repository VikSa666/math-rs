fn newton_step(f: &dyn Fn(f64) -> f64, df: &dyn Fn(f64) -> f64, x: f64) -> f64 {
    x - f(x) / df(x)
}
