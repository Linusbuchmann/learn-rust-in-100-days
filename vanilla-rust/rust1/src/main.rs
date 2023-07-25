#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("wie ist ihr namen?");
    let mut name = String::new();
    let greeting = "schön dich zu treffen";
    io::stdin().read_line(&mut name).expect("did not enter a name");
    println!("Hello {} {}", name.trim_end(), greeting);
}
