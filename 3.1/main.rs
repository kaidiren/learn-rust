use std::fmt;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point: ({}, {})", self.x, self.y)
    }
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn square(&self, p: Point, f: f32) -> Rectangle {
        let Point { x, y } = p;

        Rectangle {
            p1: p,
            p2: Point { x: x + f, y: y + f },
        }
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "p1: ({}, {}) \np2: ({}, {})",
            self.p1.x, self.p1.y, self.p2.x, self.p2.y
        )
    }
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 0.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    let new_point = Point { x: 0.1, ..point };

    println!("second point: ({}, {})", new_point.x, new_point.y);

    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("{}", _rectangle);
    println!("{}", _rectangle.rect_area());
    println!("{}", _rectangle.square(Point { x: 0.1, y: 0.1 }, 0.8f32))
}
