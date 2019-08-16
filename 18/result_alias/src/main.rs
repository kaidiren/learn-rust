use std::num::ParseIntError;

type AliasdResult<T> = Result<T, ParseIntError>;

fn multiply(first: &str, second: &str) -> AliasdResult<i32> {
    first.parse::<i32>().and_then(|first_num| {
        second
            .parse::<i32>()
            .map(|second_num| first_num * second_num)
    })
}

fn multiply_v2(first: &str, second: &str) -> Result<i32, ParseIntError> {
    let first_num = match first.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    let second_num = match second.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    Ok(first_num * second_num)
}

fn multiply_v3(f: &str, s: &str) -> Result<i32, ParseIntError> {
    let first: i64 = f.parse()?;
    let second: i64 = s.parse()?;
    Ok((first * second) as i32)
}

fn print(result: AliasdResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    println!("Hello, world!");
    print(multiply("2", "3"));
    print(multiply("2", "t"));

    print(multiply_v2("2", "4"));
    print(multiply_v2("t", "4"));
    print(multiply_v2("2", "t"));

    print(multiply_v3("10", "2"));
    print(multiply_v3("t", "2"));
}
