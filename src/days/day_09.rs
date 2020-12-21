use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/09.txt");
    // let input = "{{<ab>},{<ab>},{<ab>},{<ab>}}";

    let mut in_garbage = false;
    let mut skip = false;
    let mut total = 0;
    for c in input.chars() {
        if skip {
            skip = false;
            continue;
        }
        if in_garbage {
            match c {
                '>' => in_garbage = false,
                '!' => skip = true,
                _ => total += 1,
            }
        } else {
            match c {
                '<' => in_garbage = true,
                '!' => skip = true,
                '{' => (),
                '}' => (),
                ',' => (),
                _ => panic!("Invalid char {}", c),
            }
        }
    }
    pv!(total);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/09.txt");
    // let input = "{{<ab>},{<ab>},{<ab>},{<ab>}}";

    let mut in_garbage = false;
    let mut skip = false;
    let mut current_score = 0;
    let mut total = 0;
    for c in input.chars() {
        if skip {
            skip = false;
            continue;
        }
        if in_garbage {
            match c {
                '>' => in_garbage = false,
                '!' => skip = true,
                _ => (),
            }
        } else {
            match c {
                '<' => in_garbage = true,
                '!' => skip = true,
                '{' => current_score += 1,
                '}' => {
                    total += current_score;
                    current_score -= 1;
                }
                ',' => (),
                _ => panic!("Invalid char {}", c),
            }
        }
    }
    pv!(total);
}
