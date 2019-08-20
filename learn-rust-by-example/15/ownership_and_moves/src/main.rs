fn destory_box(c: Box<i32>) {
    println!("destory {}", c);
}

fn main() {
    println!("Hello, world!");
    let b = Box::new(5i32);
    destory_box(b);
}
