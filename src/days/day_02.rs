use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");
    //     let input = r#"5	1	9	5
    // 7	5	3
    // 2	4	6	8"#;

    let nums = input
        .lines()
        .map(|line| line.split('\t').map(|n| isize::from_str(n).unwrap()).to_vec())
        .to_vec();

    let sum: isize = nums
        .iter()
        .map(|row| {
            let mut sum = 0;
            for (i, &n) in row.iter().enumerate() {
                for &n2 in row.iter().skip(i + 1) {
                    if n % n2 == 0 {
                        sum += n / n2;
                    } else if n2 % n == 0 {
                        sum += n2 / n;
                    }
                }
            }
            sum
        })
        .sum();

    pv!(sum);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");
    //     let input = r#"5	1	9	5
    // 7	5	3
    // 2	4	6	8"#;

    let nums = input
        .lines()
        .map(|line| line.split('\t').map(|n| isize::from_str(n).unwrap()).to_vec())
        .to_vec();

    let sum: isize = nums
        .iter()
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum();

    let sum = pv!(sum);
}
