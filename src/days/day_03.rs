use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");
    // let input = r#"1024"#;

    let num = parse(input);

    let mut values: HashMap<(isize, isize), isize> = HashMap::new();
    values.insert((0, 0), 1);
    values.insert((1, 0), 1);

    let mut current = 1isize;
    let mut x = 1isize;
    let mut y = 0isize;
    let mut dir = 0;
    while current < num {
        match dir {
            0 => y -= 1,
            1 => x += 1,
            2 => y += 1,
            3 => x -= 1,
            n => panic!("Invalid dir: {}", n),
        }
        current = values
            .iter()
            .filter(|(&pos, _)| moore_i(pos, (x, y)) == 1)
            .map(|(_, &v)| v)
            .sum();
        values.insert((x, y), current);

        if dir != 1 && x.abs() == y.abs() || dir == 1 && x == y + 1 {
            dir = (dir + 3) % 4;
        }
    }

    pv!(current);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");
    // let input = r#"1024"#;

    let num = isize::from_str(input).unwrap();

    let mut current = 3;
    let mut x: isize = 1;
    let mut y: isize = -1;
    let mut dir = 3;
    while current < num {
        match dir {
            0 => y -= 1,
            1 => x += 1,
            2 => y += 1,
            3 => x -= 1,
            n => panic!("Invalid dir: {}", n),
        }
        current += 1;
        if dir != 1 && x.abs() == y.abs() || dir == 1 && x == y + 1 {
            dir = (dir + 3) % 4;
        }
    }

    pv!(x.abs() + y.abs());
}
