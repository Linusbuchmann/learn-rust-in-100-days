fn main() {
    let age: &str = "47";
    let mut age: u32 = age.trim().parse().expect("fuck you!");
    println!("{}", age);
}
