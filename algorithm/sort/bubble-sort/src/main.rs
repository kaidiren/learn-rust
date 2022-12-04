fn sort(mut v: Vec<i32>) -> Vec<i32> {
    for i in 0..v.len() {
        for j in 0..(v.len() - 1 - i) {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
            }
        }
    }
    v
}

fn main() {
    let t = vec![-5, 4, -4, 4, 3, 2, 1, 5, 6, 5644, 76, 689, -2];
    println!("{:?}", sort(t));
}
