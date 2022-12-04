fn quick_sort<T: Ord + Copy>(data: &mut [T]) {
    let len = data.len();
    if len < 2 {
        return;
    }
    let end = len - 1;

    let mut cur = 0;
    for i in 0..end {
        if data[i] < data[end] {
            data.swap(cur, i);
            cur += 1;
        }
    }
    data.swap(cur, end);

    if cur > 1 {
        quick_sort(&mut data[0..cur]);
    }
    if cur + 1 < len {
        quick_sort(&mut data[cur + 1..len]);
    }
}

fn main() {
    let mut v = vec![
        1, 5, 9, 10, 2, 4, 3, 0, -2, 14, 15, 16, 100, 8, 7, 6, 11, 12, 13,
    ];
    quick_sort(&mut v);
    println!("{:?}", v);
}
