fn sort(v: Vec<i32>) -> Vec<i32> {
    let mut t: Vec<i32> = vec![];
    for i in 0..v.len() {
        if t.len() == 0 {
            t.push(v[0])
        } else {
            let mut inserted = false;
            for j in 0..t.len() {
                if v[i] < t[j] {
                    t.insert(j, v[i]);
                    inserted = true;
                    break;
                }
            }
            if !inserted {
                t.push(v[i]);
            }
        }
    }
    t
}

fn main() {
    let t = vec![-5, 4, -4, 4, 3, 2, 1, 5, 6, 5644, 76, 689, -2];
    println!("{:?}", sort(t));
}
