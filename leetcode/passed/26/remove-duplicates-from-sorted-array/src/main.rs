use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let hash_set: HashSet<&mut i32> = nums.into_iter().collect();
        hash_set.len() as i32
    }
}

fn main() {
    let nums: &mut Vec<i32> = &mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    println!("{}", Solution::remove_duplicates(nums));
}
