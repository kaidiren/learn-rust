fn main() {
    println!("Hello, world!");
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("{:?}", collected_iterator);

    let mut xs = vec![1i32, 2, 3];
    for x in xs.iter() {
        println!("{}", x);
    }

    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Updated vector: {:?}", xs);
    xs.push(0);
    xs.push(1);
    xs.pop();
    xs.remove(0);
    println!("Updated vector: {:?}", xs);
}
