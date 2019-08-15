macro_rules! calcuate {
    (eval $e: expr) => {{
        {
            let val: usize = $e;
            println!("{} = {}", stringify!($e), val);
        }
    }};

    (eval $e:expr, $(eval $es:expr), +) => {{
        calcuate! { eval $e }
        calcuate! { $(eval $es), + }
    }};
}

fn main() {
    println!("Hello, world!");
    calcuate! {
        eval 1 + 2
    }

    calcuate! {
        eval (1 + 2) * (3 / 4)
    }

    calcuate! {
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}
