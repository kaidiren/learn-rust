static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    println!("Hello, world!");
    {
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);
    }

    {
        let num = 9;
        let coerced_static = coerce_static(&num);
        println!("coerce_static: {}", coerced_static);
    }
    println!("NUM: {}", NUM);
    println!("NUM: {}", NUM);
}
