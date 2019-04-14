#![allow(unused)]

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

macro_rules! pv {
    ($e: expr) => {
        println!("{} at line {}: {}", stringify!($e), line!(), $e);
    };
}

fn main() {
    let input = include_str!("input/day_02.txt");
    /*
        let input = r#"5	1	9	5
    7	5	3
    2	4	6	8"#;
        // */

    let nums = input
        .lines()
        .map(|line| line.split('\t').map(|n| i32::from_str(n).unwrap()).to_vec())
        .to_vec();

    let sum: i32 = nums
        .iter()
        .map(|row| {
            let mut sum = 0;
            for (i, &n) in row.iter().enumerate() {
                for &n2 in row.iter().skip(i + 1) {
                    if n % n2 == 0 {
                        sum += n / n2;
                    } else if n2 % n == 0 {
                        sum += n2 / n;
                    }
                }
            }
            sum
        })
        .sum();

    pv!(sum);
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
