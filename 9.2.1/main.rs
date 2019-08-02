fn main() {
    use std::mem;

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
        count
    };
    println!("inc: {}", inc());
    println!("inc: {}", inc());
    inc();
    inc();

    let moveable = Box::new(3);

    let consume = || {
        println!("`moveable`: {:?}", moveable);
        mem::drop(moveable);
    };

    consume();

    let haystack = vec![1, 2, 3];
    let contains = |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    println!("There're {} elements in vec", haystack.len());
}
