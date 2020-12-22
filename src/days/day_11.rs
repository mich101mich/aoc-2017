use crate::utils::*;

const N: usize = 0;
const NE: usize = 1;
const SE: usize = 2;
const S: usize = 3;
const SW: usize = 4;
const NW: usize = 5;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");

    let parsed = input.split(',').map(|dir| match dir {
        "n" => N,
        "ne" => NE,
        "se" => SE,
        "s" => S,
        "sw" => SW,
        "nw" => NW,
        s => panic!("Invalid Direction {}", s),
    });

    fn distance(mut count: [isize; 6]) -> isize {
        static COMBINATIONS: [(usize, usize, usize); 4] =
            [(NW, NE, N), (SW, SE, S), (N, SE, NE), (S, NW, SW)];

        static CANCELS: [(usize, usize); 3] = [(N, S), (NE, SW), (NW, SE)];

        for (i1, i2, o) in COMBINATIONS.iter().copied() {
            let n = count[i1].min(count[i2]);
            count[i1] -= n;
            count[i2] -= n;
            count[o] += n;
        }
        for (a, b) in CANCELS.iter().copied() {
            let ns = count[a].min(count[b]);
            count[a] -= ns;
            count[b] -= ns;
        }
        count.iter().sum()
    }
    let mut count = [0; 6];
    let mut max = std::isize::MIN;
    for d in parsed {
        count[d] += 1;
        let dist = distance(count);
        max = max.max(dist);
    }
    pv!(max);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");

    let parsed = input.split(',').map(|dir| match dir {
        "n" => 0,
        "s" => 1,
        "ne" => 2,
        "sw" => 3,
        "nw" => 4,
        "se" => 5,
        s => panic!("Invalid Direction {}", s),
    });

    let mut count = [0; 6];
    for d in parsed {
        count[d] += 1;
    }

    for i in 0..2 {
        let n = count[i + 2].min(count[i + 4]);
        count[i + 2] -= n;
        count[i + 4] -= n;
        count[i] += n;
    }
    for i in 0..3 {
        let ns = count[i * 2].min(count[i * 2 + 1]);
        count[i * 2] -= ns;
        count[i * 2 + 1] -= ns;
    }
    pv!(count);
    pv!(count.iter().sum::<isize>());
}
