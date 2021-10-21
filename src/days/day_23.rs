use crate::utils::*;

#[allow(unused)]
pub fn run() {
    let mut h = 0;
    let start = 57 * 100 + 100000;
    let end = start + 17000;
    for b in (start..=end).step_by(17) {
        let mut found = false;
        for d in 2..=b / 2 {
            if b % d == 0 && b / d >= 2 {
                found = true;
                break;
            }
        }
        if (found) {
            h += 1;
        }
    }
    pv!(h);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");

    let parsed = input.lines().map(Instruction::from).to_vec();

    let mut registers: HashMap<char, isize> = HashMap::new();
    let mut sounds: Vec<isize> = vec![];
    let mut instr = 0isize;

    let mut res = 0;
    loop {
        if instr < 0 || instr >= parsed.len() as isize {
            break;
        }
        match &parsed[instr as usize] {
            Set(a, b) => {
                let v = b.get(&registers);
                *registers.entry(*a).or_insert(0) = v;
            }
            Add(a, b) => {
                let v = b.get(&registers);
                *registers.entry(*a).or_insert(0) += v;
            }
            Sub(a, b) => {
                let v = b.get(&registers);
                *registers.entry(*a).or_insert(0) -= v;
            }
            Mul(a, b) => {
                let v = b.get(&registers);
                *registers.entry(*a).or_insert(0) *= v;
                res += 1;
            }
            Mod(a, b) => {
                let v = b.get(&registers);
                *registers.entry(*a).or_insert(0) %= v;
            }
            JumpGZ(a, b) => {
                if a.get(&registers) > 0 {
                    instr += b.get(&registers) - 1;
                }
            }
            JumpNZ(a, b) => {
                if a.get(&registers) != 0 {
                    instr += b.get(&registers) - 1;
                }
            }
        }
        instr += 1;
    }
    pv!(res);
}

#[derive(Clone, Copy, Debug)]
enum Value {
    Num(isize),
    Var(char),
}
use Value::*;
impl std::str::FromStr for Value {
    type Err = <isize as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 {
            let c = s.chars().next().unwrap();
            if c.is_alphabetic() {
                return Ok(Var(c));
            }
        }
        s.parse().map(Num)
    }
}
impl RegexRepresentation for Value {
    const REGEX: &'static str = r"[a-zA-Z]|\-?\d+";
}
impl Value {
    fn get(&self, reg: &HashMap<char, isize>) -> isize {
        match self {
            Num(n) => *n,
            Var(v) => reg.get(v).copied().unwrap_or(0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Set(char, Value),
    Add(char, Value),
    Sub(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    JumpGZ(Value, Value),
    JumpNZ(Value, Value),
}
use Instruction::*;
impl From<&str> for Instruction {
    fn from(l: &str) -> Self {
        if let Some((a, b)) = scanf!(l, "set {} {}", char, Value) {
            Set(a, b)
        } else if let Some((a, b)) = scanf!(l, "add {} {}", char, Value) {
            Add(a, b)
        } else if let Some((a, b)) = scanf!(l, "sub {} {}", char, Value) {
            Sub(a, b)
        } else if let Some((a, b)) = scanf!(l, "mul {} {}", char, Value) {
            Mul(a, b)
        } else if let Some((a, b)) = scanf!(l, "mod {} {}", char, Value) {
            Mod(a, b)
        } else if let Some((a, b)) = scanf!(l, "jgz {} {}", Value, Value) {
            JumpGZ(a, b)
        } else if let Some((a, b)) = scanf!(l, "jnz {} {}", Value, Value) {
            JumpNZ(a, b)
        } else {
            panic!("Invalid Instruction {}", l)
        }
    }
}
