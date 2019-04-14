#![allow(unused)]

use std::collections::{HashMap, HashSet};

macro_rules! pv {
    ($e: expr) => {
        println!("{} at line {}: {}", stringify!($e), line!(), $e);
    };
}

fn main() {
    let input = include_str!("input/day_01.txt");
    //let input = "91212129";

    let chars = input.chars().to_vec();

    let sum = chars
        .iter()
        .zip(chars.iter().skip(1).chain(chars.iter()))
        .filter(|(a, b)| a == b)
        .map(|c| *c.0 as usize - '0' as usize)
        .sum::<usize>();

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
