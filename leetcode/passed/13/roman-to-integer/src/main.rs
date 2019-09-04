struct Solution;

//impl Solution {
//    pub fn roman_to_int(s: String) -> i32 {
//        let chars = s.chars().collect::<Vec<_>>();
//        let mut num = 0;
//        let max = chars.len();
//        let mut i = 0;
//        while i < max {
//            match chars[i] {
//                'I' => {
//                    if i < max - 1 {
//                        match chars[i + 1] {
//                            'V' => {
//                                num += 4;
//                                i += 1;
//                            }
//                            'X' => {
//                                num += 9;
//                                i += 1;
//                            }
//                            _ => {
//                                num += 1;
//                            }
//                        }
//                    } else {
//                        num += 1;
//                    }
//                }
//                'V' => {
//                    num += 5;
//                }
//                'X' => {
//                    if i < max - 1 {
//                        match chars[i + 1] {
//                            'L' => {
//                                num += 40;
//                                i += 1;
//                            }
//                            'C' => {
//                                num += 90;
//                                i += 1;
//                            }
//                            _ => {
//                                num += 10;
//                            }
//                        }
//                    } else {
//                        num += 10;
//                    }
//                }
//                'L' => {
//                    num += 50;
//                }
//                'C' => {
//                    if i < max - 1 {
//                        match chars[i + 1] {
//                            'D' => {
//                                num += 400;
//                                i += 1;
//                            }
//                            'M' => {
//                                num += 900;
//                                i += 1;
//                            }
//                            _ => {
//                                num += 100;
//                            }
//                        }
//                    } else {
//                        num += 100;
//                    }
//                }
//                'D' => {
//                    num += 500;
//                }
//                'M' => {
//                    num += 1000;
//                }
//                _ => {}
//            }
//            i = i + 1;
//        }
//
//        num
//    }
//}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        fn char_to_int(c: u8) -> i32 {
            match c {
                b'I' => 1,
                b'V' => 5,
                b'X' => 10,
                b'L' => 50,
                b'C' => 100,
                b'D' => 500,
                b'M' => 1000,
                _ => unreachable!(),
            }
        }
        let s = s.as_bytes();
        let mut num = 0;
        for (i, c) in s.iter().enumerate() {
            let tmp = char_to_int(*c);
            if i == s.len() - 1 || tmp >= char_to_int(s[i + 1]) {
                num += tmp;
            } else {
                num -= tmp;
            }
        }
        num
    }
}
fn main() {
    println!("{}", Solution::roman_to_int(String::from("IV")));
}
