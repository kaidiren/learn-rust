use std::fmt;

struct Message<'a> {
    data: &'a str,
}

impl<'a> Drop for Message<'a> {
    fn drop(&mut self) {
        println!("Dropping Message with data `{}`!", self.data);
    }
}

impl<'a> fmt::Display for Message<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

fn main() {
    println!("Hello, world!");
    let c = Message { data: "aaaa" };
    let d = Message { data: "bbbb" };
    println!("{}", c);
    println!("{}", d);
}
