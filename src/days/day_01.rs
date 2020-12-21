use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/01.txt");
    // let input = "12131415";

    let chars = input.chars().to_vec();

    let sum = chars
        .iter()
        .zip(chars.iter().skip(chars.len() / 2).chain(chars.iter()))
        .filter(|(a, b)| a == b)
        .map(|c| *c.0 as usize - '0' as usize)
        .sum::<usize>();

    pv!(sum);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/01.txt");
    // let input = "91212129";

    let chars = input.chars().to_vec();

    let sum = chars
        .iter()
        .zip(chars.iter().skip(1).chain(chars.iter()))
        .filter(|(a, b)| a == b)
        .map(|c| *c.0 as usize - '0' as usize)
        .sum::<usize>();

    pv!(sum);
}
