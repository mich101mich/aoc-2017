use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");
    // let input = "flqrgnkx";

    let mut grid = (0..128)
        .map(|row| format!("{}-{}", input, row))
        .map(|s| knot_hash(&s))
        .map(|hash| {
            hash.iter()
                .flat_map(|byte| (0..8).rev().map(move |i| ((byte >> i) & 1) == 1))
                .to_vec()
        })
        .to_vec();

    let mut regions = get_grid(0, 128, 128);
    let mut current_region = 0;
    let iter = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, v)| (x, y, *v)));

    let neigh = ManhattanNeighborhood::new(128, 128);

    for (x, y, v) in iter {
        if !v {
            continue;
        }
        if regions[y][x] > 0 {
            continue;
        }
        current_region += 1;
        let mut next = vec![(x, y)];
        while let Some(p) = next.pop() {
            if regions[p.1][p.0] > 0 {
                continue;
            }
            regions[p.1][p.0] = current_region;
            for o in neigh.get_all_neighbors(p) {
                if grid[o.1][o.0] && regions[o.1][o.0] == 0 {
                    next.push(o);
                }
            }
        }
    }
    pv!(current_region);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");
    // let input = "flqrgnkx";

    let mut grid = (0..128)
        .map(|row| format!("{}-{}", input, row))
        .map(|s| knot_hash(&s))
        .map(|hash| {
            hash.iter()
                .flat_map(|byte| (0..8).rev().map(move |i| ((byte >> i) & 1) == 1))
                .to_vec()
        })
        .to_vec();

    let count = grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|v| **v)
        .count();
    pv!(count);
}

fn knot_hash(input: &str) -> Vec<u8> {
    let mut list = (0u8..=255).to_vec();
    let mut pos = 0;
    let mut skip = 0;
    let n = list.len();

    let mut lengths = input.chars().map(|c| c as usize).to_vec();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);

    for _ in 0..64 {
        for length in lengths.iter().copied() {
            if length == 0 {
                pos = (pos + length + skip) % n;
                skip += 1;
                continue;
            }
            let end = (pos + length - 1) % n;
            let mut a = pos;
            let mut b = end;
            while a != b {
                list.swap(a, b);
                a = (a + 1) % n;
                if a == b {
                    break;
                }
                b = (b + n - 1) % n;
            }
            pos = (pos + length + skip) % n;
            skip += 1;
        }
    }
    list.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, v| acc ^ *v))
        .collect()
}
