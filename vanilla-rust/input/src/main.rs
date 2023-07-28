use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
fn main() {
    println!("whats your name??");
    let mut name = String::new();
    let greeting = "Nice to meet you (im joking fuck you)";
    io::stdin().read_line(&mut name).expect("fuck you");
    println!("hello {}, {}", name.trim_end(), greeting);
}
