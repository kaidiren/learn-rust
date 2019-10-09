struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut keys: HashSet<i32> = HashSet::new();
        keys.insert(0);
        fn find_and_save(key: i32, keys: &mut HashSet<i32>, rooms: &Vec<Vec<i32>>) {
            for k in rooms[key as usize].iter() {
                if keys.insert(*k) {
                    find_and_save(*k, keys, rooms);
                }
            }
        }
        for (i, room) in rooms.iter().enumerate() {
            if keys.contains(&(i as i32)) {
                for key in room.iter() {
                    keys.insert(*key);
                    find_and_save(*key, &mut keys, &rooms);
                }
            }
        }
        //        println!("{:?}", keys);
        //        println!("{:?}", rooms);
        keys.len() == rooms.len()
    }
}

fn main() {
    // let t: Vec<Vec<i32>> = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];
    let t: Vec<Vec<i32>> = vec![
        vec![13],
        vec![15, 29, 15, 22],
        vec![5, 18, 9],
        vec![7],
        vec![27],
        vec![27],
        vec![6, 28],
        vec![26],
        vec![34],
        vec![1, 44, 11],
        vec![8, 36],
        vec![17, 35],
        vec![11, 45, 46, 10, 49],
        vec![19, 38, 47, 39],
        vec![20, 30],
        vec![34],
        vec![32, 31],
        vec![25, 19, 21, 29],
        vec![36],
        vec![],
        vec![38],
        vec![2, 13, 17, 47],
        vec![12],
        vec![49, 46],
        vec![],
        vec![40],
        vec![],
        vec![39, 16, 24],
        vec![24, 41],
        vec![14, 3, 40],
        vec![14, 43],
        vec![],
        vec![3, 20, 23],
        vec![37, 48],
        vec![6, 10],
        vec![26, 1, 4],
        vec![],
        vec![41, 45],
        vec![23, 33],
        vec![],
        vec![22, 18, 37],
        vec![4, 33, 43],
        vec![28, 31, 42],
        vec![30, 48],
        vec![16, 35],
        vec![5, 8, 44],
        vec![2, 25],
        vec![9, 21, 42],
        vec![7, 12, 32],
        vec![],
    ];
    println!("{}", Solution::can_visit_all_rooms(t));
}
