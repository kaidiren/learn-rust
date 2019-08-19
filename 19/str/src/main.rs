fn main() {
    println!("Hello, world!");
    let p = "aaaa";
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);
    println!("p: {}", p);

    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();
    let mut string = String::new();
    for c in chars {
        // 在字符串的尾部插入一个字符
        println!("{}", c);
        string.push(c);
        // 在字符串尾部插入一个字符串
        string.push_str(", ");
    }
    println!("{}", string);
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    let alice = String::from("I like dogs");
    let bob = alice.replace("dog", "cat");
    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}
