fn main() {
    let arr_it: [i32; 4] = [1, 2, 3, 4];
    for val in arr_it.iter() {
        println!("{}", val);
    }

    let mut iter1: std::slice::Iter<i32> = arr_it.iter();
    println!("1st: {:?}", iter1.next().unwrap());
}
