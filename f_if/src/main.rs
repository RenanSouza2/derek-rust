use std::cmp::Ordering;

fn main() {
    let age: i32 = 8;
    if (age >= 1) && (age <= 18) {
        println!("Important Birthday");
    } else if (age == 21) || (age == 50) {
        println!("Almost Important Birthday");
    } else if age >= 65 {
        println!("Less Important Birthday");
    } else {
        println!("Not an Important Birthday");
    }

    let mut my_age = 47;
    let can_vote = if my_age >= 18 { true } else { false };
    println!("Can vote : {}", can_vote);

    my_age = 18;
    match my_age {
        1..19 => print!("Important Birthday"),
        21 | 50 => println!("Almost Important Birthday"),
        65..=i32::MAX => println!("Less Important Birthday"),
        _ => println!("Not an Important Birthday"),
    };

    my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal => println!("You Gained the right to vote"),
    }
}
