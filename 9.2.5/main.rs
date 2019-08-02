fn create_fn() -> Box<Fn()> {
    let text = "Fn".to_owned();
    Box::new(move || println!("this is a: {}", text))
}

fn create_fn_mut() -> Box<FnMut()> {
    let text = "FnMut".to_owned();
    Box::new(move || println!("this is a: {}", text))
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fn_mut();
    fn_plain();
    fn_mut();
}
