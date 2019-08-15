use std::ops::{Add, Mul, Sub};

// block
// expr 用于表达式
// ident 用于变量名或函数名
// item
// pat (模式 pattern)
// path
// stmt (语句 statement)
// tt (标记树 token tree)
// ty (类型 type)

macro_rules! assert_equal_len {
    ($a: ident, $b: ident, $func:ident, $op:tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?}: dimension mismatch: {:?} {:?} {:?}",
            stringify!($func),
            ($a.len(),),
            stringify!($op),
            ($b.len(),)
        );
    };
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        #[allow(dead_code)]
        fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);
            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
            }
        }
    };
}

op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

mod test {
    macro_rules! test {
        ($func: ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = std::iter::repeat($x).take(size).collect();
                    let y: Vec<_> = std::iter::repeat($y).take(size).collect();
                    let z: Vec<_> = std::iter::repeat($z).take(size).collect();
                    println!("x: {:?}, y: {:?}, z: {:?}", x, y, z);
                    super::$func(&mut x, &y);
                    assert_eq!(x, z);
                }
            }
        };
    }

    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);
}

fn main() {
    println!("Hello, world!");
}
