fn main() {
    let mut count = 0_u32;
    println!("let's count until infinity!");

    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);

        if count == 5 {
            println!("ok, that's enough");
            break;
        }
    }
}
