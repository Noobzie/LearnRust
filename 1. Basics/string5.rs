// string5.rs
fn main() {
    let mut s = String::new();
    // Initially empty!
    s.push('H');
    s.push_str("ello");
    s.push(' ');
    s += "World!"; // short for 'push_str'
    // remove teh last char
    s.pop();

    assert_eq!(s, "Hello World");
}