#[derive(Debug, Clone, Copy)]
struct Nil;

#[derive(Debug, Clone)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    println!("Hello, world!");
    let nil = Nil;
    let copied_nil = nil;
    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("ori: {:?}", pair);

    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    let cloned_pair = moved_pair.clone();

    drop(moved_pair);

    println!("clone: {:?}", cloned_pair);
}
