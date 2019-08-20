#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}

#[derive(Debug)]
struct Hello {
    x: i32,
}

impl Default for Hello {
    fn default() -> Self {
        Self { x: 10 }
    }
}

fn main() {
    println!("Hello, world!");
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
    let h: Hello = Default::default();
    println!("h is {:?}", h);
    println!("h.x is {:?}", h.x);
}
