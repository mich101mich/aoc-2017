use crate::utils::*;
use std::sync::mpsc::*;

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
    Sound(Value),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Recover(Value),
    Jump(Value, Value),
}
use Instruction::*;
impl From<&str> for Instruction {
    fn from(l: &str) -> Self {
        if let Some(s) = scanf!(l, "snd {}", Value) {
            Sound(s)
        } else if let Some(s) = scanf!(l, "rcv {}", Value) {
            Recover(s)
        } else if let Some((a, b)) = scanf!(l, "set {} {}", char, Value) {
            Set(a, b)
        } else if let Some((a, b)) = scanf!(l, "add {} {}", char, Value) {
            Add(a, b)
        } else if let Some((a, b)) = scanf!(l, "mul {} {}", char, Value) {
            Mul(a, b)
        } else if let Some((a, b)) = scanf!(l, "mod {} {}", char, Value) {
            Mod(a, b)
        } else if let Some((a, b)) = scanf!(l, "jgz {} {}", Value, Value) {
            Jump(a, b)
        } else {
            panic!("Invalid Instruction {}", l)
        }
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");

    let (send_1, recv_2) = channel::<isize>();
    let (send_2, recv_1) = channel::<isize>();
    let mut channels = [Some((send_1, recv_1)), Some((send_2, recv_2))];

    let parsed = input.lines().map(Instruction::from).to_vec();

    let mut handles = vec![];
    for (id, ch) in channels.iter_mut().enumerate() {
        let (send, recv) = ch.take().unwrap();
        let parsed = parsed.clone();
        let handle = std::thread::spawn(move || {
            let mut registers: HashMap<char, isize> = HashMap::new();
            registers.insert('p', id as isize);
            let mut sounds: Vec<isize> = vec![];
            let mut instr = 0isize;
            let mut sent = 0;
            let timeout = std::time::Duration::from_millis(100);
            loop {
                if instr < 0 || instr >= parsed.len() as isize {
                    break;
                }
                match &parsed[instr as usize] {
                    Sound(a) => {
                        send.send(a.get(&registers));
                        if id == 1 {
                            sent += 1;
                        }
                    }
                    Recover(a) => {
                        let a = match a {
                            Var(v) => v,
                            _ => panic!("Recover integer"),
                        };
                        let val = match recv.recv_timeout(timeout) {
                            Ok(v) => v,
                            _ => break,
                        };
                        *registers.entry(*a).or_insert(0) = val;
                    }
                    Set(a, b) => {
                        let v = b.get(&registers);
                        *registers.entry(*a).or_insert(0) = v;
                    }
                    Add(a, b) => {
                        let v = b.get(&registers);
                        *registers.entry(*a).or_insert(0) += v;
                    }
                    Mul(a, b) => {
                        let v = b.get(&registers);
                        *registers.entry(*a).or_insert(0) *= v;
                    }
                    Mod(a, b) => {
                        let v = b.get(&registers);
                        *registers.entry(*a).or_insert(0) %= v;
                    }
                    Jump(a, b) => {
                        if a.get(&registers) > 0 {
                            instr += b.get(&registers) - 1;
                        }
                    }
                }
                instr += 1;
            }
            sent
        });
        handles.push(handle);
    }
    for handle in handles {
        pv!(handle.join().unwrap());
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");

    let parsed = input.lines().map(Instruction::from).to_vec();

    let mut registers: HashMap<char, isize> = HashMap::new();
    let mut sounds: Vec<isize> = vec![];
    let mut instr = 0isize;
    loop {
        if instr < 0 || instr >= parsed.len() as isize {
            break;
        }
        match &parsed[instr as usize] {
            Sound(a) => {
                sounds.push(a.get(&registers));
            }
            Recover(a) => {
                if a.get(&registers) != 0 {
                    pv!(sounds.pop());
                    return;
                }
            }
            Set(a, b) => {
                let v = b.get(&registers);
                *registers.entry(*a).or_insert(0) = v;
            }
            Add(a, b) => {
                let v = b.get(&registers);
                *registers.entry(*a).or_insert(0) += v;
            }
            Mul(a, b) => {
                let v = b.get(&registers);
                *registers.entry(*a).or_insert(0) *= v;
            }
            Mod(a, b) => {
                let v = b.get(&registers);
                *registers.entry(*a).or_insert(0) %= v;
            }
            Jump(a, b) => {
                if a.get(&registers) > 0 {
                    instr += b.get(&registers) - 1;
                }
            }
        }
        instr += 1;
    }
}
