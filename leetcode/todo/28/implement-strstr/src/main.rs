struct Solution;
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.chars().collect::<Vec<char>>();
        let needle = needle.chars().collect::<Vec<char>>();
        if needle.len() == 0 || haystack.len() < needle.len() {
            return 0;
        }
        let mut i = 0;
        while i < haystack.len() - needle.len() {
            if haystack[i] == needle[0] {
                let mut equal = true;
                for v in needle.iter() {
                    if &haystack[i] != v {
                        equal = false;
                        i += 1;
                        break;
                    }
                    i += 1;
                }
                if equal {
                    return (i - needle.len()) as i32;
                }
            } else {
                i += 1;
            }
        }
        0
    }
}
fn main() {
    println!(
        "{}",
        Solution::str_str(String::from("hello"), String::from("lo"))
    );
}
