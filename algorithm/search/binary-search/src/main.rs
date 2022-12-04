fn find(v: &Vec<i32>, t: i32) -> i32 {
    let mut head: usize = 0;
    let mut tail: usize = v.len() - 1;
    if t > v[v.len() - 1] || t < v[0] {
        return -1;
    }
    while head < tail {
        let index = (head + tail) / 2;
        if v[index] > t {
            tail = index;
        } else if v[index] < t {
            head = index;
        } else {
            head = index;
            break;
        }
    }
    head as i32
}

pub fn binary_search<T>(arr: &[T], target: &T) -> Result<usize, usize>
where
    T: PartialOrd,
{
    let mut size = arr.len();
    if size == 0 {
        return Err(0);
    }

    let mut base = 0_usize;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        if arr[mid] <= *target {
            base = mid;
        }
        size -= half;
        println!("base: {}, mid: {}, size: {}", base, mid, size);
    }

    if arr[base] == *target {
        Ok(base)
    } else {
        Err(base + (arr[base] < *target) as usize)
    }
}

fn main() {
    println!("Hello, world!");
    let t = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    println!("{}", find(&t, -1));
    println!("{}", find(&t, 13));
    println!("{}", find(&t, 5));
    println!("{:?}", binary_search(&t, &13));
    println!("{:?}", binary_search(&t, &-1));
    println!("{:?}", binary_search(&t, &8));
    println!("{:?}", binary_search(&t, &1));
    println!("{:?}", binary_search(&t, &1));
}
