//slice2.rs
fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);

    println!("first {:?}", first);
    println!("last {:?}", last);

    println!("First {} {}", first.is_some(), first.is_none());
    println!("Last {} {}", last.is_some(), last.is_none());
    println!("First value {}", first.unwrap());

    let maybe_last = slice.get(5);
    let _last = if maybe_last.is_some() {
        *maybe_last.unwrap()
    } else {
        -1
    };

    let last1 = *slice.get(5).unwrap_or(&-1);
    println!("Last1 {}", last1);
}