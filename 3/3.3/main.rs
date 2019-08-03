static LANGUAGE: &'static str = "Rust";

const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;
    let LAN: String = "Rust".to_owned();
    println!("this is {}", LANGUAGE);
    println!("this is {}", LAN);
    println!("the THRESHOLD is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}
