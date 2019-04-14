#![allow(unused)]

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

macro_rules! pv {
    ($e: expr) => {
        println!("{} at line {}: {}", stringify!($e), line!(), $e);
    };
}

fn main() {
    let input = include_str!("input/day_06.txt");
    /*
    let input = "0\t2\t7\t0";
    // */
    let mut banks = input
        .split('\t')
        .map(|line| i32::from_str(line).unwrap_or_else(|_| panic!("{}", line)))
        .to_vec();

    let mut seen = HashMap::new();
    let mut iterations = 0;
    let len = banks.len();

    loop {
        let mut mi = 0;
        for i in 1..len {
            if banks[i] > banks[mi] {
                mi = i;
            }
        }
        let count = banks[mi];
        banks[mi] = 0;
        for di in 0..count {
            banks[(mi + 1 + di as usize) % len] += 1;
        }

        iterations += 1;

        let s = format!("{:?}", banks);
        if seen.contains_key(&s) {
            println!("{}", iterations - seen[&s]);
            break;
        }
        seen.insert(s, iterations);
    }

    pv!(iterations);
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
