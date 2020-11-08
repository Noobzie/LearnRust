// vec2.rs
fn dump(arr: &[i32]) {
    println!("Arr is {:?}", arr);
}

fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    dump(&v);

    let slice = &v[1..];
    println!("Slice is {:?}", slice);
}