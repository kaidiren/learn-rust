fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("find the sum if all the squared odd numbers under 1000");
    let upper = 1000;
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("sum is: {}", acc);

    let sum_of_suqared_odd_numbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n| n < upper)
        .filter(|&n| is_odd(n))
        .fold(0, |sum, i| sum + i);
    println!("function style: {}", sum_of_suqared_odd_numbers);
}
