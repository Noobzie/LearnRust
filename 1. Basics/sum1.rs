// sum1.rs
fn main() {
    let sum: i32 = (0..5).sum();
    println!("Sum was {}", sum);

    let sum: i64 = [10, 20, 30].iter().sum();
    println!("Sum was {}", sum);
}