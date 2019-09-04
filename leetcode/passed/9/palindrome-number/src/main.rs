pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let o: String = x.to_string();
    let r = o.chars().rev().collect::<String>();
    o == r
}

fn main() {
    println!("Hello, world!");
    println!("{}", is_palindrome(123));
}
