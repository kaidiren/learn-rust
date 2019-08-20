fn main() {
    let logical: bool = true;
    println!("{}", logical);

    let a_float: f64 = 1.0;
    println!("{}", a_float);
    let an_integer = 5i32;
    println!("{}", an_integer);

    let default_folat = 3.0;
    println!("{}", default_folat);
    let default_integer = 7;
    println!("{}", default_integer);

    let mut inferred_type = 12;
    println!("{}", inferred_type);
    inferred_type = 13i64;
    println!("{}", inferred_type);

    let mut mutable = 12;
    println!("{}", mutable);
    mutable = 13;
    println!("{}", mutable);

    let mutable = true;
    println!("{}", mutable);
}

// 主要的两个注释方式
// 1. 写在冒号后边 或者写在变量后边
// 2. 可以用 let 覆盖之前的变量
