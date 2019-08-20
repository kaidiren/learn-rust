use std::process::Command;

fn main() {
    println!("Hello, world!");
    let mut child = Command::new("ls").arg("-lh").spawn().unwrap();
    let result = child.wait().unwrap();
    println!("{:?}", result);
    println!("reached end of main");
}
