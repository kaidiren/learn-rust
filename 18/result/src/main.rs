fn multiply(first: &str, second: &str) -> i32 {
    let first_num = first.parse::<i32>().unwrap();
    let second_num = second.parse::<i32>().unwrap();
    first_num * second_num
}

fn main() {
    println!("Hello, world!");
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);
}
