use std::string::ToString;

struct Circle {
    radius: i32,
}

// impl ToString for Circle {
//     fn to_string(&self) -> String {
//         format!("Circle of radius {:?}", self.radius)
//     }
// }

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Circle of radius from display {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
    println!("{}", circle);
}

// 竟然是冲突的实现一个就可以了
