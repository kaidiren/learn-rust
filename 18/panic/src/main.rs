fn give_princess(gift: &str) {
    if gift == "snake" {
        panic!("AAAAAAA");
    } else {
        println!("I love {}s !!!", gift);
    }
}

fn main() {
    println!("Hello, world!");
    give_princess("apple");
    give_princess("snake");
}
