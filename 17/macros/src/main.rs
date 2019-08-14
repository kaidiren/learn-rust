macro_rules! say_hello {
    () => {
        println!("Hello");
    };
}

fn main() {
    println!("Hello, world!");
    say_hello!();
}
