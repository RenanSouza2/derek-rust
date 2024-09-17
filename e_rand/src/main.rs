use rand::Rng;

fn main() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random : {}", random_num);
}
