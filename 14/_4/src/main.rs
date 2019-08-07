use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}
impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.length
    }
}

impl HasArea for Triangle {
    fn area(&self) -> f64 {
        self.height * self.length
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

#[derive(Debug)]
struct Triangle {
    length: f64,
    height: f64,
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let t = Triangle {
        length: 3.0,
        height: 4.0,
    };
    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));
    println!("Hello, world!");

    print_debug(&t);
    println!("Area: {}", area(&t));
}
