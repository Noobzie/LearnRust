// search1.rs
use std::f64::consts;

fn main() {
    let pi: f64 = 3.1416;
    let x = pi/2.0;
    let cosine = x.cos();

    println!("Cosine is {}", cosine);

    let x1 = 2.0 * consts::PI;
    let abs_difference = (x1.cos() - 1.0).abs();
    assert!(abs_difference < 1e-10);
}

