fn create_box() {
    let _box = Box::new(3i32);
}

#[derive(Debug)]
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being droped");
    }
}

fn main() {
    println!("Hello, world!");
    let _box2 = Box::new(5i32);
    {
        let _box = Box::new(4i32);
    }

    for _ in 0u32..1_000 {
        create_box();
    }

    let x = ToDrop;
    println!("Made a ToDrop");
    println!("{:?}", x);
}
