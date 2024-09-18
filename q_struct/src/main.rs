use std::f32::consts::PI;

fn main() {
    struct Customer {
        _name: String,
        address: String,
        _balance: f32,
    }

    let mut bob = Customer {
        _name: String::from("Bob Smith"),
        address: String::from("Some address"),
        _balance: 234.50,
    };

    bob.address = String::from("he moved");

    struct Rectangle1<T, U> {
        _length: T,
        _height: U,
    }
    let _rec = Rectangle1 {
        _length: 4, 
        _height: 10.5
    };

    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct  Rectangle {length: f32, width: f32}
    struct Ellipse {length: f32, width: f32}

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle{length, width};
        }
        fn area(&self) -> f32 {
            self.length * self.width
        }
    }

    impl Shape for Ellipse {
        fn new(length: f32, width: f32) -> Ellipse {
            return Ellipse{length, width};
        }
        fn area(&self) -> f32 {
            self.length * self.width * PI / 4.0
        }
    }

    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Ellipse = Shape::new(10.0, 10.0);
    println!("Rec Are : {}", rec.area());
    println!("Circ Are : {}", circ.area());

}
