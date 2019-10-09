struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() <= 1 {
            return 0;
        }
        let mut max_index = 0;
        let mut rain = 0;
        for (i, v) in height.iter().enumerate() {
            if *v >= height[max_index] {
                max_index = i;
            }
        }
        println!("max index: {}", max_index);

        let left = height[0..max_index].to_vec();
        let mut right = height[max_index + 1..height.len()].to_vec();
        right.reverse();
        right.insert(0, 0);
        fn count_rain(h: Vec<i32>) -> i32 {
            if h.len() <= 1 {
                return 0;
            }
            let mut max = h[0];
            let mut rain = 0;
            for (i, v) in h.iter().enumerate() {
                if v > &max {
                    max = *v;
                }
                if v < &max {
                    rain += max - *v;
                }
                println!("max: {} index: {} value: {} rain: {}", max, i, v, rain);
            }
            rain
        }
        rain += count_rain(left);
        rain += count_rain(right);
        rain
    }
}

fn main() {
    let t = vec![4, 2, 3];
    println!("rain: {}", Solution::trap(t));
}
