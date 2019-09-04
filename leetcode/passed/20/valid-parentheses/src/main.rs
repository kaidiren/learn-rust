struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut t: Vec<char> = vec![];
        let chars = s.chars().collect::<Vec<_>>();
        fn is_pair(a: &char, b: &char) -> bool {
            match a {
                '(' => match b {
                    ')' => true,
                    _ => false,
                },
                '[' => match b {
                    ']' => true,
                    _ => false,
                },
                '{' => match b {
                    '}' => true,
                    _ => false,
                },
                _ => false,
            }
        }
        for c in &chars {
            if t.len() == 0 {
                t.push(*c);
                continue;
            }

            if is_pair(&t[t.len() - 1], c) {
                t.pop();
                continue;
            } else {
                t.push(*c);
            }
        }
        t.len() == 0
    }
}

fn main() {
    println!("{}", Solution::is_valid(String::from("()(){}{]}")));
}
