fn say_hello() {
    println!("Hello");
}

fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn get_sum_2(x: i32, y: i32) -> i32 {
    x + y
}

fn get_sum_3(x: i32, y: i32) -> i32 {
    return x + y;
}

fn get_2(x: i32) -> (i32, i32) {
    return (x + 1, x + 2);
}

fn sum_vec(vec: &[i32]) -> i32 {
    let mut sum = 0;
    for num in vec {
        sum += num;
    }
    return sum;
}

fn main() {
    say_hello();
    get_sum(5, 4);
    println!("{}", get_sum_2(5, 4));
    println!("{}", get_sum_3(5, 4));
    let (val_1, val_2) = get_2(3);
    println!("Nums: {} {}", val_1, val_2);

    let num_list = vec![1, 2, 3, 4, 5];
    println!("Sum of list = {}", sum_vec(&num_list));
}
