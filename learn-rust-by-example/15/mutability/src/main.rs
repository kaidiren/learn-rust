#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    book.title = "rkd";
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    println!("Hello, world!");
    let immutbook = Book {
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };
    let mut mutabook = immutbook;
    borrow_book(&immutbook);
    borrow_book(&mutabook);
    new_edition(&mut mutabook);
}
