use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");

    let mut parsed = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), if c == '#' { 2 } else { 0 }))
        })
        .to_map();

    let mut pos = (
        (input.lines().count() / 2) as isize,
        (input.lines().next().unwrap().chars().count() / 2) as isize,
    );
    let mut dir = Dir::Up;

    let mut infected = 0;
    for _ in 0..10000000 {
        let current = parsed.entry(pos).or_insert(0);
        dir = match *current {
            0 => dir.counter_clockwise(),
            1 => {
                infected += 1;
                dir
            }
            2 => dir.clockwise(),
            3 => dir.clockwise().clockwise(),
            c => panic!("Invalid State {}", c),
        };
        *current = (*current + 1) % 4;
        pos = dir.add_delta(pos);
    }
    pv!(infected);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");

    let mut parsed = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .to_set();

    let mut pos = (
        (input.lines().count() / 2) as isize,
        (input.lines().next().unwrap().chars().count() / 2) as isize,
    );
    let mut dir = Dir::Up;

    let mut infected = 0;
    for _ in 0..10000 {
        if parsed.contains(&pos) {
            dir = dir.clockwise();
            parsed.remove(&pos);
        } else {
            dir = dir.counter_clockwise();
            parsed.insert(pos);
            infected += 1;
        }
        pos = dir.add_delta(pos);
    }
    pv!(infected);
}
