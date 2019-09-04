fn reverse(x: i32) -> i32 {
    let s: String = x.to_string();
    let mut m = s.chars().rev().collect::<String>();
    if x < 0 {
        m.truncate(m.len() - 1);
    }
    let mut m = m.parse::<i32>().unwrap_or(0);
    if x < 0 {
        m = m * -1;
    }
    m
}

fn main() {
    println!("Hello, world!");
    reverse(123);
    let x = reverse(1534236469);
    println!("{}", x);
}
