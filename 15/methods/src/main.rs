struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }

    fn print<'a>(&'a self) {
        println!("print {}", self.0);
    }
}

struct My(i32);

impl My {
    fn add_one(&mut self) {
        self.0 += 1;
    }
    fn print(&self) {
        println!("my print {}", self.0);
    }
}

fn main() {
    println!("Hello, world!");
    let mut x = Owner(5);
    x.print();
    x.add_one();
    x.print();

    let mut my = My(6);
    my.print();
    my.add_one();
    my.print();
}
