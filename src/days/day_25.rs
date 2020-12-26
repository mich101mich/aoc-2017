use crate::utils::*;

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/25.txt");

    let mut lines = input.lines().to_vec();
    let mut i = 3;

    let mut state = scanf!(lines[0], "Begin in state {}.", char);
    let goal = scanf!(
        lines[1],
        "Perform a diagnostic checksum after {} steps.",
        usize
    );

    let mut states = HashMap::new();

    while i < lines.len() {
        let name = scanf!(lines[i], "In state {}:", char);
        // If the current value is 0:
        let v0 = scanf!(lines[i + 2], "    - Write the value {}.", usize);
        let r0 = lines[i + 3] == "    - Move one slot to the right.";
        let s0 = scanf!(lines[i + 4], "    - Continue with state {}.", char);
        // If the current value is 1:
        let v1 = scanf!(lines[i + 6], "    - Write the value {}.", usize);
        let r1 = lines[i + 7] == "    - Move one slot to the right.";
        let s1 = scanf!(lines[i + 8], "    - Continue with state {}.", char);
        states.insert(name, [(v0, r0, s0), (v1, r1, s1)]);
        i += 10;
    }

    let mut tape_left = Vec::with_capacity(100_000);
    let mut tape_right = Vec::with_capacity(100_000);
    let mut current = 0;
    let mut count = 0;

    loop {
        let (v, r, s) = states[&state][current];
        current = v;
        if r {
            tape_left.push(current);
            current = tape_right.pop().unwrap_or(0);
        } else {
            tape_right.push(current);
            current = tape_left.pop().unwrap_or(0);
        }
        state = s;
        count += 1;
        if count == goal {
            break;
        }
    }
    let sum = tape_left.iter().sum::<usize>() + current + tape_right.iter().sum::<usize>();
    pv!(sum);
}
