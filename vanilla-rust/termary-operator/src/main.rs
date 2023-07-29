fn main() {
    let mut my_age = 4;
    let can_vote = if my_age > 18{
        true
    }   else{
        false
    };
    println!("can vote : {}", can_vote);
}