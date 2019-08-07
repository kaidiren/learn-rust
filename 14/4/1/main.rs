struct C;

struct B;

struct T;

trait Red {}
trait Blue {}

impl Red for C {}

impl Blue for B {}

fn red<T: Red>(_: &T) -> &'static str {
    "Red"
}

fn blue<T: Blue>(_: &T) -> &'static str {
    "Blue"
}

fn main() {
    let c = C;
    let b = B;
    let t = T;
    println!(" a c is {}", red(&c));
    println!("a b jay is {}", blue(&b));
}
