use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");
    // let input = ;

    let parsed = input
        .lines()
        .filter_map(|l| {
            scanf!(
                l,
                "{} {} {} if {} {} {}",
                String,
                String,
                isize,
                String,
                String,
                isize
            )
        })
        .to_vec();

    let mut registers = HashMap::<String, isize>::new();
    let mut max = std::isize::MIN;

    for (var, instr, val, cond_var, op, comp) in parsed {
        let cond_val = registers.get(&cond_var).copied().unwrap_or(0);
        let ok = match op.as_str() {
            "<" => cond_val < comp,
            ">" => cond_val > comp,
            "<=" => cond_val <= comp,
            ">=" => cond_val >= comp,
            "==" => cond_val == comp,
            "!=" => cond_val != comp,
            o => panic!("Invalid comp: {}", o),
        };
        if ok {
            let var = registers.entry(var).or_insert(0);
            if instr == "inc" {
                *var += val;
            } else {
                *var -= val;
            }
            max = max.max(*var);
        }
    }

    pv!(max);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");
    // let input = ;

    let parsed = input
        .lines()
        .filter_map(|l| {
            scanf!(
                l,
                "{} {} {} if {} {} {}",
                String,
                String,
                isize,
                String,
                String,
                isize
            )
        })
        .to_vec();

    let mut registers = HashMap::<String, isize>::new();

    for (var, instr, val, cond_var, op, comp) in parsed {
        let cond_val = registers.get(&cond_var).copied().unwrap_or(0);
        let ok = match op.as_str() {
            "<" => cond_val < comp,
            ">" => cond_val > comp,
            "<=" => cond_val <= comp,
            ">=" => cond_val >= comp,
            "==" => cond_val == comp,
            "!=" => cond_val != comp,
            o => panic!("Invalid comp: {}", o),
        };
        if ok {
            let var = registers.entry(var).or_insert(0);
            if instr == "inc" {
                *var += val;
            } else {
                *var -= val;
            }
        }
    }

    let result = registers.values().max().unwrap();
    pv!(result);
}
