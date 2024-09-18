fn _print_str(x: String) {
    println!("A string {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    x
}

fn change_string(name:&mut String) {
    name.push_str(" is happy");
    print!("Message : {}", name);
}

fn main() {
    let  str1 = String::from("world");
    let _str2 = str1.clone();
    let  str3 = print_return_str(str1);
    println!("str3 : {}", str3);

    let mut str4 = String::from("Renan");
    change_string(& mut str4);
}
