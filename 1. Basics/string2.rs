// string2.rs
fn main() {
    let text = "static";
    let string = "dynamice".to_string();

    let text_s = &text[1..];
    let string_s = &string[2..4];

    println!("Slices {:?} {:?}", text_s, string_s);
}