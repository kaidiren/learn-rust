mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct CloseBox<T> {
        contents: T,
    }

    impl<T> CloseBox<T> {
        pub fn new(contents: T) -> CloseBox<T> {
            CloseBox { contents: contents }
        }

        pub fn see(&self) -> &T {
            &self.contents
        }
    }
}

fn main() {
    let open_box = my::OpenBox {
        contents: "public information",
    };
    println!("the open box contains: {}", open_box.contents);
    println!("the open box contains: {}", open_box.contents);

    let _closed_box = my::CloseBox::new("classified information");

    println!("the closed box contents: {}", _closed_box.see());
    println!("the closed box contents: {}", _closed_box.see());
}
