use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");

    let parsed = input
        .lines()
        .map(|l| l.split(' ').last().unwrap())
        .map(parse_u)
        .to_vec();
    let mut a = parsed[0];
    let mut b = parsed[1];

    let mut same_count = 0;
    for _ in 0..5_000_000 {
        a = (a * 16807) % 2147483647;
        while a % 4 != 0 {
            a = (a * 16807) % 2147483647;
        }
        b = (b * 48271) % 2147483647;
        while b % 8 != 0 {
            b = (b * 48271) % 2147483647;
        }

        if (a & 0xffff) == (b & 0xffff) {
            same_count += 1;
        }
    }
    pv!(same_count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");

    let parsed = input
        .lines()
        .map(|l| l.split(' ').last().unwrap())
        .map(parse_u)
        .to_vec();
    let mut a = parsed[0];
    let mut b = parsed[1];

    let mut same_count = 0;
    for _ in 0..40_000_000 {
        a = (a * 16807) % 2147483647;
        b = (b * 48271) % 2147483647;

        if (a & 0xffff) == (b & 0xffff) {
            same_count += 1;
        }
    }
    pv!(same_count);
}
