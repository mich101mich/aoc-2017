use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord([isize; 3]);

impl std::ops::Index<usize> for Coord {
    type Output = isize;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}
impl std::ops::IndexMut<usize> for Coord {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}
impl std::ops::AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
    }
}
impl RegexRepresentation for Coord {
    const REGEX: &'static str =
        const_format::concatcp!(isize::REGEX, ",", isize::REGEX, ",", isize::REGEX);
}
impl std::str::FromStr for Coord {
    type Err = <isize as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(',').map(|s| s.parse::<isize>());
        Ok(Self([
            s.next().unwrap()?,
            s.next().unwrap()?,
            s.next().unwrap()?,
        ]))
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");

    let mut parsed = input
        .lines()
        .filter_map(|l| scanf!(l, "p=<{}>, v=<{}>, a=<{}>", Coord, Coord, Coord))
        .to_vec();

    let mut set = HashMap::with_capacity(parsed.len());
    for iter in 0..1000 {
        set.clear();
        for (p, v, a) in parsed.iter_mut() {
            *v += *a;
            *p += *v;
            *set.entry(*p).or_insert(0) += 1;
        }
        parsed.retain(|(p, _, _)| set[&p.clone()] == 1);
    }
    pv!(parsed.len());
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");
    let mut parsed = input
        .lines()
        .filter_map(|l| scanf!(l, "p=<{}>, v=<{}>, a=<{}>", Coord, Coord, Coord))
        .to_vec();

    for iter in 0..10_000 {
        for (p, v, a) in parsed.iter_mut() {
            *v += *a;
            *p += *v;
        }
    }
    let min = parsed
        .iter()
        .enumerate()
        .min_by_key(|(_, (p, _, _))| manhattan_3d_i((p[0], p[1], p[2]), (0, 0, 0)))
        .unwrap();
    pv!(min.0);
}
