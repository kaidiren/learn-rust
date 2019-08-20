fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("find 2 in vec1: {:?}", vec1.iter().find(|&&x| x == 2));
    println!("find 2 in vec2: {:?}", vec2.into_iter().find(|&x| x == 2));
}
