use std::num::ParseIntError;

fn multiply_v1(first: &str, second: &str) -> Result<i32, ParseIntError> {
    match first.parse::<i32>() {
        Ok(first_num) => match second.parse::<i32>() {
            Ok(second_num) => Ok(first_num * second_num),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn multiply_v2(first: &str, second: &str) -> Result<i32, ParseIntError> {
    first.parse::<i32>().and_then(|first_num| {
        second
            .parse::<i32>()
            .map(|second_num| first_num * second_num)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    println!("Hello, world!");
    let twenty = multiply_v1("10", "2");
    print(twenty);

    // 这种情况下就会提供一条更有用的错误信息。
    let tt = multiply_v1("t", "2");
    print(tt);

    let twenty = multiply_v2("10", "2");
    print(twenty);

    // 这种情况下就会提供一条更有用的错误信息。
    let tt = multiply_v2("t", "2");
    print(tt);
}
