struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

fn main() {
    println!("Hello, world!");
    let mut sequence = 0..3;
    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("-------------------");
    for i in 0..3 {
        println!("> {}", i);
    }

    println!("-------------------");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    println!("-------------------");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    println!("-------------------");
    let array = [1u32, 3, 3, 7];

    println!("{:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }

    let mut x: i32 = 1;
    println!("{}", x);
    x = -1;
    println!("{}", x);
}
