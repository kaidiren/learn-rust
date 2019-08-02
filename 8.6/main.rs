fn main() {
    let optional = Some(7);
    match optional {
        Some(i) => {
            println!("this is a really long string and `{:?}`", i);
        }
        _ => {}
    }

    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!(" > 9");
            optional = None;
        } else {
            println!("`i` is {:?}, try again", i);
            optional = Some(i + 1);
        }
    }
}
