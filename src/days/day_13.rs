use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");

    let parsed = input
        .lines()
        .map(|l| l.split(": ").map(parse_u).to_vec())
        .map(|a| (a[0], a[1] * 2 - 2))
        .to_vec();

    let start = (0..)
        .find(|start| {
            parsed
                .iter()
                .all(|(offset, cycle)| (start + offset) % cycle != 0)
        })
        .unwrap();
    pv!(start);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");

    let parsed = input
        .lines()
        .map(|l| l.split(": ").map(parse_u).to_vec())
        .map(|a| (a[0], a[1] * 2 - 2))
        .to_vec();

    let severity: usize = parsed
        .iter()
        .filter(|(offset, cycle)| offset % cycle == 0)
        .map(|(offset, cycle)| offset * (cycle + 2) / 2)
        .sum();
    pv!(severity)
}
