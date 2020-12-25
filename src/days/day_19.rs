use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");

    let parsed = input.lines().map(|l| l.chars().to_vec()).to_vec();
    let mut pos = (
        parsed[0]
            .iter()
            .enumerate()
            .find(|(_, c)| **c == '|')
            .unwrap()
            .0,
        0,
    );
    let mut dir = Dir::Down;

    let mut count = 0;
    loop {
        count += 1;
        pos = dir.add_delta_u(pos);
        match parsed[pos.1][pos.0] {
            '|' | '-' => {}
            '+' => {
                let mut next_dir = dir.counter_clockwise();
                let mut next_pos = next_dir.add_delta_u(pos);
                while parsed[next_pos.1][next_pos.0] == ' ' {
                    next_dir = next_dir.clockwise();
                    next_pos = next_dir.add_delta_u(pos);
                }
                dir = next_dir;
            }
            c if c.is_alphabetic() => {
                print!("{}", c);
            }
            ' ' => break,
            c => panic!("Encountered unexpected {}", c),
        }
    }
    println!();
    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");

    let parsed = input.lines().map(|l| l.chars().to_vec()).to_vec();
    let mut pos = (
        parsed[0]
            .iter()
            .enumerate()
            .find(|(_, c)| **c == '|')
            .unwrap()
            .0,
        0,
    );
    let mut dir = Dir::Down;

    loop {
        pos = dir.add_delta_u(pos);
        match parsed[pos.1][pos.0] {
            '|' | '-' => {}
            '+' => {
                let mut next_dir = dir.counter_clockwise();
                let mut next_pos = next_dir.add_delta_u(pos);
                while parsed[next_pos.1][next_pos.0] == ' ' {
                    next_dir = next_dir.clockwise();
                    next_pos = next_dir.add_delta_u(pos);
                }
                dir = next_dir;
            }
            c if c.is_alphabetic() => {
                print!("{}", c);
            }
            ' ' => break,
            c => panic!("Encountered unexpected {}", c),
        }
    }
    println!();
}
