use crate::utils::*;

fn run_prog(programs: &mut Vec<char>, input: &str) {
    for dance_move in input.split(',') {
        let (action, args) = dance_move.split_at(1);
        match action {
            "s" => {
                let pos = programs.len() - parse_u(args);
                let (back, front) = programs.split_at(pos);
                let mut new_programs = front.to_vec();
                new_programs.extend_from_slice(back);
                *programs = new_programs;
            }
            "x" => {
                let mut iter = args.split('/').map(parse_u);
                programs.swap(iter.next().unwrap(), iter.next().unwrap());
            }
            "p" => {
                let mut iter = args.chars();
                let a = iter.next().unwrap();
                iter.next().unwrap(); // '/'
                let b = iter.next().unwrap();
                for prog in programs.iter_mut() {
                    if *prog == a {
                        *prog = b;
                    } else if *prog == b {
                        *prog = a;
                    }
                }
            }
            x => panic!("Unexpected dance move {}", x),
        };
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");
    let prog_count = 16u8;

    // let input = "s1,x3/4,pe/b";
    // let prog_count = 5u8;

    let initial = (0..prog_count).map(|i| (b'a' + i) as char).to_vec();

    let iterations = 1_000_000_000;

    let mut programs = initial.clone();

    let mut i = 0;
    while i < iterations {
        if i != 0 && programs == initial {
            let cycle = i;
            while i + cycle < iterations {
                i += cycle;
            }
        }
        run_prog(&mut programs, input);
        i += 1;
    }

    println!("programs: {}", programs.iter().collect::<String>());
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");

    let mut programs = (0..16).map(|i| (b'a' + i) as char).to_vec();
    pv!(programs);
    for dance_move in input.split(',') {
        let (action, args) = dance_move.split_at(1);
        match action {
            "s" => {
                let pos = programs.len() - parse_u(args);
                let (back, front) = programs.split_at(pos);
                let mut new_programs = front.to_vec();
                new_programs.extend_from_slice(back);
                programs = new_programs;
            }
            "x" => {
                let mut iter = args.split('/').map(parse_u);
                programs.swap(iter.next().unwrap(), iter.next().unwrap());
            }
            "p" => {
                let mut iter = args.chars();
                let a = iter.next().unwrap();
                iter.next().unwrap(); // '/'
                let b = iter.next().unwrap();
                for prog in programs.iter_mut() {
                    if *prog == a {
                        *prog = b;
                    } else if *prog == b {
                        *prog = a;
                    }
                }
            }
            x => panic!("Unexpected dance move {}", x),
        };
    }
    let res: String = programs.iter().collect();
    pv!(res);
}
