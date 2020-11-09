// string1.rs
fn dump(s: &str) {
    println!("str '{}'", s);
}

fn main() {
    let text = "Hello dolly";
    let s = text.to_string();

    dump(text);
    dump(&s);
}