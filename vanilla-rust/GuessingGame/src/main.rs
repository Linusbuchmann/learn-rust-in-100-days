use std::io;
use rand::Rng;

fn main() {
    println!("try to guess the random number");
    loop {
        let random_number = rand::thread_rng().gen_range(1..20);
        let mut guessed_number: String = String::new();
        let random_number_string=random_number.to_string();
        io::stdin().read_line(&mut guessed_number).expect("fuck you");
        if random_number_string != guessed_number.trim()
            {
            println!("the right answer was: {}, you guessed {}", random_number, guessed_number);
            println!("try again");
            }
        else {
            { print!("You got it right\n"); }
            break;
        }
    }
}
