#![allow(unused)]

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

macro_rules! pv {
    ($e: expr) => {
        println!("{} at line {}: {}", stringify!($e), line!(), $e);
    };
}

fn main() {
    let input = include_str!("input/day_07.txt");
    /*
    let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
    // */
    let mut parents = HashMap::new();
    let mut children = HashMap::new();
    let mut disks = HashMap::new();

    for line in input.lines() {
        let mut iter = line.split(' ');
        let name = iter.next().unwrap();
        let weight = iter.next().unwrap();

        let weight = i32::from_str(&weight[1..weight.len() - 1]).unwrap();
        disks.insert(name, weight);

        if let Some(_) = iter.next() {
            // arrow
            let child_disks = iter
                .map(|c| {
                    if c.ends_with(',') {
                        &c[..c.len() - 1]
                    } else {
                        c
                    }
                })
                .to_vec();

            for &child in &child_disks {
                parents.insert(child, name);
            }

            children.insert(name, child_disks);
        }
    }
    let mut root = *parents.keys().next().unwrap();
    while parents.contains_key(root) {
        root = parents[root];
    }

    fn get_weight(name: &str, d: &HashMap<&str, i32>, c: &HashMap<&str, Vec<&str>>) -> i32 {
        let mut me = d[name];
        if let Some(children) = c.get(name) {
            me += children
                .iter()
                .map(|&child| get_weight(child, d, c))
                .sum::<i32>();
        }
        me
    };

    for (disk, my_children) in children.iter() {
        let child_weights = my_children
            .iter()
            .map(|&child| get_weight(child, &disks, &children))
            .to_vec();
        if child_weights.iter().any(|w| *w != child_weights[0]) {
            pv!(disk);
            for c in 0..my_children.len() {
                println!("{}: {} ({})", my_children[c], child_weights[c], disks[my_children[c]]);
            }
        }
    }
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
