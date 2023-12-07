use arithmetic_yukinari::{add, subtract};

mod sub;

fn main() {
    println!("{}", add(1, 2));
    println!("{}", subtract(1, 2));
    sub::greet();
    sub::greet2();
    sub::subsub::greet();
}