use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

pub fn show(nums: Vec<i32>) {
    for (k, v) in nums.iter().enumerate() {
        println!("{}, {}", k, v);
    }
}

pub fn two_sum_hash_map(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (k, v) in nums.iter().enumerate() {
        let another = target - *v;
        if let Some(num) = map.get(&another) {
            return vec![*num as i32, k as i32];
        }
        map.insert(v, k);
    }

    vec![]
}

fn main() {
    let result = two_sum_hash_map([2, 3, 3, 2, 4].to_vec(), 6);
    for i in result {
        println!("{}", i);
    }
}
