/// Compute numerical integral of function of one real variable using trapezoidal rule.
fn trapezoid(rhs: fn(f64) -> f64, a: f64, b: f64, n: i32) -> f64 {
    // Width of one trapezoid
    let h = (b - a) / n as f64;
    h * (0.5 * rhs(a) + (1..n).fold(0.0, |acc, i| acc + rhs(a + i as f64 * h)) + 0.5 * rhs(b))
}

// right hand side function.
fn rhs(t: f64) -> f64 {
    3.0 * t.powi(2) * std::f64::consts::E.powf(t.powi(3))
}

fn main() {
    println!("{}", trapezoid(rhs, 0.0, 1.0, 100));
}
