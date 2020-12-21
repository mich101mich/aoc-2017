use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/04.txt");
    // let input = r#"aa bb cc dd aa"#;

    let count = input
        .lines()
        .filter(|line| {
            line.split(' ').count()
                == line
                    .split(' ')
                    .map(|word| {
                        let mut chars = word.chars().to_vec();
                        chars.sort_unstable();
                        chars.into_iter().collect::<String>()
                    })
                    .collect::<HashSet<_>>()
                    .len()
        })
        .count();

    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/04.txt");
    // let input = r#"aa bb cc dd aa"#;

    let count = input
        .lines()
        .filter(|line| line.split(' ').count() == line.split(' ').collect::<HashSet<_>>().len())
        .count();

    pv!(count);
}
