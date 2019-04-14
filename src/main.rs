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
    let input = r#"5"#;
    // */
    let num = i32::from_str(input).unwrap();

    let mut values: HashMap<(i32, i32), i32> = HashMap::new();
    values.insert((0, 0), 1);
    values.insert((1, 0), 1);

    let mut current: i32 = 1;
    let mut x: i32 = 1;
    let mut y: i32 = 0;
    let mut dir = 0;
    while current < num {
        match dir {
            0 => y -= 1,
            1 => x += 1,
            2 => y += 1,
            3 => x -= 1,
            n => panic!("Invalid dir: {}", n),
        }
        current = values
            .iter()
            .filter(|(&pos, _)| moore(pos, (x, y)) == 1)
            .map(|(_, &v)| v)
            .sum();
        
        values.insert((x, y), current);

        if dir != 1 && x.abs() == y.abs() || dir == 1 && x == y + 1 {
            dir = (dir + 3) % 4;
        }
    }

    pv!(current);
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
