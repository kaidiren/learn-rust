mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    use std::fmt;

    impl fmt::Display for Breakfast {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Breakfast:\ntoast: {}\n seasonal_fruit: {}",
                self.toast, self.seasonal_fruit
            )
        }
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn print(&self) {
            println!(
                "toast: {}, seasonal_fruit: {}",
                self.toast, self.seasonal_fruit
            );
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    meal.print();
    println!("Display: {}", meal);
}

fn main() {
    println!("Hello, world!");
    eat_at_restaurant();
}
