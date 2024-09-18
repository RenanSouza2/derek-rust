use std::f32::consts::PI;

fn main() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("Some address"),
        balance: 234.50,
    };

    bob.address = String::from("he moved");

    struct Rectangle1<T, U> {
        length: T,
        height: U,
    }
    let rec = Rectangle1 {
        length: 4, 
        height: 10.5
    };

    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct  Rectangle {length: f32, width: f32};
    struct Circle {length: f32, width: f32};

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle{length, width};
        }
        fn area(&self) -> f32 {
            self.length * self.width
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle{length, width};
        }
        fn area(&self) -> f32 {
            self.length * self.length * PI / 4.0
        }
    }

    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);
    println!("Rec Are : {}", rec.area());
    println!("Circ Are : {}", circ.area());

}
