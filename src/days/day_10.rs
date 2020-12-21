use crate::utils::*;

fn knot_hash(input: &str) -> String {
    let mut list = (0..256).to_vec();
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
        .map(|v| format!("{:02x}", v))
        .collect()
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let hash = knot_hash(input);

    pv!(hash);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");
    let num = 256;

    // let input = "3,4,1,5";
    // let num = 5;

    let lengths = input.split(',').map(parse_u).to_vec();

    let mut list = (0..num).to_vec();
    let mut pos = 0;
    let mut skip = 0;
    let n = list.len();

    for length in lengths {
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
    pv!(list[0] * list[1]);
}
