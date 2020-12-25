use crate::utils::*;

#[derive(Clone, Copy, Debug)]
enum Value {
    Num(isize),
    Var(char),
}
use Value::*;
impl From<String> for Value {
    fn from(s: String) -> Self {
        if let Ok(num) = s.parse() {
            Num(num)
        } else {
            Var(s.chars().next().unwrap())
        }
    }
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
        if let Ok(s) = scan_fmt!(l, "snd {}", String) {
            Sound(s.into())
        } else if let Ok(s) = scan_fmt!(l, "rcv {}", String) {
            Recover(s.into())
        } else if let Ok((a, b)) = scan_fmt!(l, "set {} {}", char, String) {
            Set(a, b.into())
        } else if let Ok((a, b)) = scan_fmt!(l, "add {} {}", char, String) {
            Add(a, b.into())
        } else if let Ok((a, b)) = scan_fmt!(l, "mul {} {}", char, String) {
            Mul(a, b.into())
        } else if let Ok((a, b)) = scan_fmt!(l, "mod {} {}", char, String) {
            Mod(a, b.into())
        } else if let Ok((a, b)) = scan_fmt!(l, "jgz {} {}", String, String) {
            Jump(a.into(), b.into())
        } else {
            panic!("Invalid Instruction {}", l)
        }
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");

    let (send_1, recv_2) = std::sync::mpsc::channel::<isize>();
    let (send_2, recv_1) = std::sync::mpsc::channel::<isize>();
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
            loop {
                if instr < 0 || instr >= parsed.len() as isize {
                    break;
                }
                match &parsed[instr as usize] {
                    Sound(a) => {
                        send.send(a.get(&registers));
                        if id == 1 {
                            sent += 1;
                            pv!(sent);
                        }
                    }
                    Recover(a) => {
                        let a = match a {
                            Var(v) => v,
                            _ => panic!("Recover integer"),
                        };
                        *registers.entry(*a).or_insert(0) = recv.recv().unwrap();
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
