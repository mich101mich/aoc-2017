#![allow(unused)]

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

macro_rules! pv {
    ($e: expr) => {
        println!("{} at line {}: {}", stringify!($e), line!(), $e);
    };
}

fn main() {
    let input = include_str!("input/day_03.txt");
    /*
    let input = r#"1024"#;
    // */
    let num = i32::from_str(input).unwrap();

    let mut current = 3;
    let mut x: i32 = 1;
    let mut y: i32 = -1;
    let mut dir = 3;
    while current < num {
        match dir {
            0 => y -= 1,
            1 => x += 1,
            2 => y += 1,
            3 => x -= 1,
            n => panic!("Invalid dir: {}", n),
        }
        current += 1;
        if dir != 1 && x.abs() == y.abs() || dir == 1 && x == y + 1 {
            dir = (dir + 3) % 4;
        }
    }

    pv!(x.abs() + y.abs());
}

trait IterExt<T> {
    fn to_vec(self) -> Vec<T>;
}
impl<I, T> IterExt<T> for I
where
    I: Iterator<Item = T>,
{
    fn to_vec(self) -> Vec<T> {
        self.collect()
    }
}
