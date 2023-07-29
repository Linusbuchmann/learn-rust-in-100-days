fn main(){
    let age2 = 14;
    match age2 {
        1..=18=> println!("Important Birthday"),
        21..= 50 => println!("Important birthday"),
        _ => println!("not an important birthday"),
    }
}