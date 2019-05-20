#![allow(unreachable_code)]
fn main() {
    'outer: loop {
        println!("enterd the outer loop");

        'inner: loop {
            println!("enterd the inner loop");

            break 'outer;
        }

        println!("this point will never be reached");
    }

    println!("exited the outer loop");
}
