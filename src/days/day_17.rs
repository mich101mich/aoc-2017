use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");
    // let input = "3";

    let parsed = parse_u(input);

    let mut count = 50000000;
    let mut len = 1;
    let mut pos = 0;
    let mut result = 0;
    for i in 1..=count {
        pos = (pos + parsed) % len + 1;
        len += 1;
        if pos == 1 {
            result = i;
        }
    }
    pv!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");
    // let input = "3";

    let parsed = parse_u(input);

    let mut count = 2017;
    let mut buffer = VecDeque::with_capacity(count + 1);
    buffer.push_back(0usize);

    let mut pos = 0;
    for i in 1..=count {
        pos = (pos + parsed) % buffer.len();
        buffer.insert(pos + 1, i);
        pos += 1;
    }
    pos = (pos + 1) % buffer.len();
    pv!(buffer[pos]);
}
