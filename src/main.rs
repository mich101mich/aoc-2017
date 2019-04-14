#![allow(unused)]

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

macro_rules! pv {
    ($e: expr) => {
        println!("{} at line {}: {}", stringify!($e), line!(), $e);
    };
}

fn main() {
    let input = include_str!("input/day_05.txt");
    /*
    let input = r#"0
3
0
1
-3"#;
    // */
    let mut lines = input
        .lines()
        .map(|line| i32::from_str(line).unwrap_or_else(|_| panic!("{}", line)))
        .to_vec();

    let mut current: i32 = 0;
    let mut steps = 0;

    while current >= 0 && current < lines.len() as i32 {
        let val = lines[current as usize];
        if val >= 3 {
            lines[current as usize] -= 1;
        } else {
            lines[current as usize] += 1;
        }
        current += val;
        steps += 1;
    }

    pv!(steps);
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

fn manhattan((ax, ay): (i32, i32), (bx, by): (i32, i32)) -> i32 {
    (ax - bx).abs() + (ay - by).abs()
}
fn moore((ax, ay): (i32, i32), (bx, by): (i32, i32)) -> i32 {
    (ax - bx).abs().max((ay - by).abs())
}
