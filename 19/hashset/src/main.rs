use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();
    assert!(a.insert(4));
    assert!(a.contains(&4));

    b.insert(5);
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    println!("union {:?}", a.union(&b).collect::<Vec<&i32>>());
    println!("difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());
    println!(
        "Intersection: {:?}",
        a.intersection(&b).collect::<Vec<&i32>>()
    );
    println!(
        "Symmetric Difference: {:?}",
        a.symmetric_difference(&b).collect::<Vec<&i32>>()
    );
}
