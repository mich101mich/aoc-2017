use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");
    // let input = "0\t2\t7\t0";

    let mut banks = input
        .split('\t')
        .map(|line| isize::from_str(line).unwrap_or_else(|_| panic!("{}", line)))
        .to_vec();

    let mut seen = HashMap::new();
    let mut iterations = 0;
    let len = banks.len();

    loop {
        let mut mi = 0;
        for i in 1..len {
            if banks[i] > banks[mi] {
                mi = i;
            }
        }
        let count = banks[mi];
        banks[mi] = 0;
        for di in 0..count {
            banks[(mi + 1 + di as usize) % len] += 1;
        }

        iterations += 1;

        let s = format!("{:?}", banks);
        if seen.contains_key(&s) {
            println!("{}", iterations - seen[&s]);
            break;
        }
        seen.insert(s, iterations);
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");
    // let input = "0\t2\t7\t0";

    let mut banks = input
        .split('\t')
        .map(|line| isize::from_str(line).unwrap_or_else(|_| panic!("{}", line)))
        .to_vec();

    let mut seen = HashSet::new();
    let mut iterations = 0;
    let len = banks.len();

    loop {
        let mut mi = 0;
        for i in 1..len {
            if banks[i] > banks[mi] {
                mi = i;
            }
        }
        let count = banks[mi];
        banks[mi] = 0;
        for di in 0..count {
            banks[(mi + 1 + di as usize) % len] += 1;
        }

        iterations += 1;

        if !seen.insert(format!("{:?}", banks)) {
            break;
        }
    }

    pv!(iterations);
}
