fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const _PI: f32 = 3.141592;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse().expect("Age was not assigned a number");
    age += 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}
