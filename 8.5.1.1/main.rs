fn main() {
    let pair = (0, -2);
    println!("tell me about {:?}", pair);

    match pair {
        (0, y) => println!("first is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("x is {:?}, y is 0", x),
        _ => println!("...."),
    }
}
