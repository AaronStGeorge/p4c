/// Compute numerical integral of function of one real variable using the
/// composite trapezoidal rule.
fn trapezoid(rhs: fn(f64) -> f64, a: f64, b: f64, n: i32) -> f64 {
    // Width of one trapezoid
    let h = (b - a) / n as f64;
    let inner_sum = (1..n).fold(0.0, |acc, i| acc + rhs(a + h * i as f64));
    h * (0.5 * rhs(a) + inner_sum + 0.5 * rhs(b))
}

/// Compute numerical integral of function of one real variable using the
/// composite midpoint rule
fn midpoint(rhs: fn(f64) -> f64, a: f64, b: f64, n: i32) -> f64 {
    // Width of one rectangle
    let h = (b - a) / n as f64;
    h * (0..n).fold(0.0, |acc, i| acc + rhs(a + h * i as f64 + h / 2.0))
}

// 3t²e^(t^3).
fn rhs(t: f64) -> f64 {
    3.0 * t.powi(2) * std::f64::consts::E.powf(t.powi(3))
}

fn main() {
    println!("trapezoid: {}", trapezoid(rhs, 0.0, 1.0, 100));
    println!("midpoint: {}", midpoint(rhs, 0.0, 1.0, 100));
}
