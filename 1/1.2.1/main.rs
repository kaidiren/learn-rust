#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} {2:?} is the {actor:?} name, {hello:?}",
        "Slater",
        "Christan",
        actor = "actor's",
        hello = "hello"
    );
    println!("Now {:?} a will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let p = Person { name, age };

    println!("{:#?}", p)
}
