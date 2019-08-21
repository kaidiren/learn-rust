fn main() {
    println!("Hello, world!");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("end");

    let a = [10, 20, 30, 40, 50];
    for i in 0..a.len() {
        println!("{}, {}", i, a[i]);
    }

    for e in a.iter() {
        println!("e: {}", e);
    }
}
