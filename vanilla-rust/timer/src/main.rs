use std::{thread, time::Duration};

fn main() {
    loop {
        println!("hello world!");
        thread::sleep(Duration::from_millis(4000));
        println!("hello world!");
    }

}
