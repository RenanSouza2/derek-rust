fn main() {
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
    println!("Name: {}", my_tuple.1);
    let(v1, _v2, _v3) = my_tuple;
    println!("Age: {}", v1);
}
