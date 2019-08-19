use std::path::Path;

fn main() {
    println!("Hello, world!");
    let path = Path::new(".");
    let display = path.display();
    println!("{}", display);
    let new_path = path.join("a").join("../b");
    match new_path.to_str() {
        None => panic!(""),
        Some(s) => println!("new path is {}", s),
    };
}
