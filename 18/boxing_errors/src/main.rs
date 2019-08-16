use std::error;
use std::fmt;
use std::num::ParseIntError;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {
    fn description(&self) -> &str {
        "invalid first item to double"
    }
    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into())
        .and_then(|s| s.parse::<i32>().map_err(|e| e.into()).map(|i| i * 2))
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    println!("Hello, world!");
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
