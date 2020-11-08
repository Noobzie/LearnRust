// vec1.rs
fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0]; // Will panic if out-of-range
    let maybe_first = v.get(0); //Will not panic if out-of-range

    println!("V is {:?}", v.get(0));
    println!("First is {}", first);
    println!("Maybe_first is {:?}", maybe_first);

}