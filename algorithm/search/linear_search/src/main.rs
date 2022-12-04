pub fn line_search<T>(arr: &[T], target: &T) -> Option<usize>
where
    T: PartialOrd,
{
    for (index, item) in arr.iter().enumerate() {
        if item == target {
            return Some(index);
        }
    }
    None
}

pub fn line_search_v2<T>(arr: &[T], target: &T) -> Option<usize>
where
    T: PartialOrd,
{
    arr.iter().position(|x| x == target)
}

fn main() {
    println!("{:?}", line_search(&[1, 2, 3, 4], &4));
    println!("{:?}", line_search(&[1, 2, 3, 4], &1));
    println!("{:?}", line_search_v2(&[1, 2, 3, 4], &2));
}
