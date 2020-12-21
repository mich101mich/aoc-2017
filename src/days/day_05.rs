use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");
    //     let input = "0
    // 3
    // 0
    // 1
    // -3";

    let mut lines = input
        .lines()
        .map(|line| isize::from_str(line).unwrap_or_else(|_| panic!("{}", line)))
        .to_vec();

    let mut current: isize = 0;
    let mut steps = 0;

    while current >= 0 && current < lines.len() as isize {
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

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");
    //     let input = "0
    // 3
    // 0
    // 1
    // -3";

    let mut lines = input
        .lines()
        .map(|line| isize::from_str(line).unwrap_or_else(|_| panic!("{}", line)))
        .to_vec();
    let mut current: isize = 0;
    let mut steps = 0;

    while current >= 0 && current < lines.len() as isize {
        let val = lines[current as usize];
        lines[current as usize] += 1;
        current += val;
        steps += 1;
    }

    pv!(steps);
}
