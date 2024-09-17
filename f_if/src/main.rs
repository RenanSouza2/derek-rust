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

    let mut my_age =47;
    let can_vode = if my_age >= 18 {
        true
    } else {
        false
    }
}
