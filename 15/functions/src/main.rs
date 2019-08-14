fn print_one<'a>(x: &'a i32) {
    println!("x is {}", x);
}

fn print_multi<'a, 'b>(x: &'a i32, y: &'a i32) {
    println!("x is {} and y is {}", x, y);
}

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

fn add_one<'a>(x: &'a mut i32) {
    *x += 1
}

fn main() {
    println!("Hello, world!");
    let x = 7;
    let y = 9;
    print_one(&x);
    print_multi(&x, &y);
    let z = pass_x(&x, &y);
    print_one(&z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}
