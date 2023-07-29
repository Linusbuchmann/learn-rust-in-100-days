use std::io;
use rand::Rng;


fn main() {
    let random_number = rand::thread_rng().gen_range(1..20);
    let mut guessed_number: String = String::new();
    let random_number_string=random_number.to_string();
    io::stdin().read_line(&mut guessed_number).expect("fuck you");
    if random_number_string == guessed_number.trim()
        { print!("You got it right\n"); }
    else {
        println!("the right answer was: {}, you guessed {}", random_number, guessed_number);
    }

    
}
