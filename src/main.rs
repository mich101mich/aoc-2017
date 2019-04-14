#![allow(unused)]

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

macro_rules! pv {
    ($e: expr) => {
        println!("{} at line {}: {}", stringify!($e), line!(), $e);
    };
}

fn main() {
    let input = include_str!("input/day_04.txt");
    /*
    let input = r#"abcde xyz ecdab
abcde fghij
a ab abc abd abf abj"#;
    // */
    let count = input
        .lines()
        .filter(|line| {
            line.split(' ').count()
                == line
                    .split(' ')
                    .map(|word| {
                        let mut chars = word.chars().to_vec();
                        chars.sort();
                        chars.into_iter().collect::<String>()
                    })
                    .collect::<HashSet<_>>()
                    .len()
        })
        .count();

    pv!(count);
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
