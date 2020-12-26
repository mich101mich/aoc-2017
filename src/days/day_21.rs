use crate::utils::*;

fn key_generator(input: &[&[char]]) -> impl Iterator<Item = String> {
    let mut ret = vec![];
    let mut block = input.iter().map(|s| s.to_vec()).to_vec();
    for rot in 0..8 {
        if rot == 4 {
            block.reverse();
        }
        let mut s = String::new();
        for row in block.iter() {
            if !s.is_empty() {
                s.push('/');
            }
            for c in row {
                s.push(*c);
            }
        }
        ret.push(s);
        rotate_grid_clock(&mut block);
    }
    ret.into_iter()
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");
    let parsed = input
        .lines()
        .map(|l| l.split(" => ").to_vec())
        .map(|v| (v[0], v[1]))
        .to_map();

    let mut grid = ".#.\n..#\n###".lines().map(|l| l.chars().to_vec()).to_vec();

    for iter in 0..18 {
        let size = grid.len();
        let mut keys = vec![];
        if size % 2 == 0 {
            for (offset, lines) in grid.chunks(2).enumerate() {
                let mut iter = lines.iter();
                for (top, bot) in iter
                    .next()
                    .unwrap()
                    .chunks(2)
                    .zip(iter.next().unwrap().chunks(2))
                {
                    let slice = [top, bot];
                    keys.push((offset, key_generator(&slice)));
                }
            }
        } else {
            assert_eq!(size % 3, 0);
            for (offset, lines) in grid.chunks(3).enumerate() {
                let mut iter = lines.iter();
                for ((top, mid), bot) in iter
                    .next()
                    .unwrap()
                    .chunks(3)
                    .zip(iter.next().unwrap().chunks(3))
                    .zip(iter.next().unwrap().chunks(3))
                {
                    let mut slice = [top, mid, bot];
                    keys.push((offset, key_generator(&slice)));
                }
            }
        }

        let mut new_grid = vec![];
        for (mut offset, key_iter) in keys {
            let mut result = None;
            for key in key_iter {
                if let Some(value) = parsed.get(key.as_str()) {
                    result = Some(*value);
                }
            }
            let result = result.unwrap();
            let mut block = result.split('/').map(|s| s.chars().to_vec()).to_vec();
            offset *= block.len();
            while new_grid.len() < offset + block.len() {
                new_grid.push(vec![]);
            }
            new_grid[offset..]
                .iter_mut()
                .zip(block.iter_mut())
                .for_each(|(a, b)| a.append(b));
        }
        grid = new_grid;
        // for row in grid.iter() {
        //     print_arr!(row);
        // }
        // println!();
    }

    let count = grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|c| **c == '#')
        .count();
    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");
    let parsed = input
        .lines()
        .map(|l| l.split(" => ").to_vec())
        .map(|v| (v[0], v[1]))
        .to_map();

    let mut grid = ".#.\n..#\n###".lines().map(|l| l.chars().to_vec()).to_vec();

    for iter in 0..5 {
        let size = grid.len();
        let mut keys = vec![];
        if size % 2 == 0 {
            for (offset, lines) in grid.chunks(2).enumerate() {
                let mut iter = lines.iter();
                for (top, bot) in iter
                    .next()
                    .unwrap()
                    .chunks(2)
                    .zip(iter.next().unwrap().chunks(2))
                {
                    let slice = [top, bot];
                    keys.push((offset, key_generator(&slice)));
                }
            }
        } else {
            assert_eq!(size % 3, 0);
            for (offset, lines) in grid.chunks(3).enumerate() {
                let mut iter = lines.iter();
                for ((top, mid), bot) in iter
                    .next()
                    .unwrap()
                    .chunks(3)
                    .zip(iter.next().unwrap().chunks(3))
                    .zip(iter.next().unwrap().chunks(3))
                {
                    let mut slice = [top, mid, bot];
                    keys.push((offset, key_generator(&slice)));
                }
            }
        }

        let mut new_grid = vec![];
        for (mut offset, key_iter) in keys {
            let mut result = None;
            for key in key_iter {
                if let Some(value) = parsed.get(key.as_str()) {
                    result = Some(*value);
                }
            }
            let result = result.unwrap();
            let mut block = result.split('/').map(|s| s.chars().to_vec()).to_vec();
            offset *= block.len();
            while new_grid.len() < offset + block.len() {
                new_grid.push(vec![]);
            }
            new_grid[offset..]
                .iter_mut()
                .zip(block.iter_mut())
                .for_each(|(a, b)| a.append(b));
        }
        grid = new_grid;
        // for row in grid.iter() {
        //     print_arr!(row);
        // }
        // println!();
    }

    let count = grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|c| **c == '#')
        .count();
    pv!(count);
}
