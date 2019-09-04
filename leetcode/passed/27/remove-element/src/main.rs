struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut index = 0;
        while index < nums.len() {
            if nums[index] == val {
                nums.remove(index);
            } else {
                index += 1;
            }
        }
        nums.len() as i32
    }
}

fn main() {
    let nums: &mut Vec<i32> = &mut vec![1, 2, 2, 2, 2, 2, 3, 4, 5, 6, 7];
    println!("{}", Solution::remove_element(nums, 2));
}
