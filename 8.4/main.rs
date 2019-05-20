fn main() {
    // for n in 1..101 {
    //     if n % 15 == 0 {
    //         println!("fizzbuzz");
    //     } else if n % 3 == 0 {
    //         println!("fizz");
    //     } else if n % 5 == 0 {
    //         println!("buzz");
    //     } else {
    //         println!("{}", n);
    //     }
    // }

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("there is a rustacean among us!"),
            _ => println!("hello {}", name),
        }
    }

    println!("{:?}", names);

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("there is a rustacean among us!"),
            _ => println!("hello {}", name),
        }
    }

    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        match name {
            &mut "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}
// 好多语法糖和黑魔法 ╮(╯_╰)╭  _(:з」∠)_
