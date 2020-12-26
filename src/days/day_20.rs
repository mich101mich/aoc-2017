use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");

    //     let input = "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
    // p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
    // p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
    // p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>";

    let mut parsed = input
        .lines()
        .map(|l| scanf!(l, "p=<{}>, v=<{}>, a=<{}>", String, String, String))
        .map(|(p, v, a)| {
            (
                p.split(',').map(parse).to_vec(),
                v.split(',').map(parse).to_vec(),
                a.split(',').map(parse).to_vec(),
            )
        })
        .map(|(p, v, a)| (p, v, a))
        .to_vec();

    for iter in 0..1000 {
        let mut set = HashMap::with_capacity(parsed.len());
        for (p, v, a) in parsed.iter_mut() {
            for i in 0..3 {
                v[i] += a[i];
                p[i] += v[i];
            }
            *set.entry(p.clone()).or_insert(0) += 1;
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
        .map(|l| scanf!(l, "p=<{}>, v=<{}>, a=<{}>", String, String, String))
        .map(|(p, v, a)| {
            (
                p.split(',').map(parse).to_vec(),
                v.split(',').map(parse).to_vec(),
                a.split(',').map(parse).to_vec(),
            )
        })
        .map(|(p, v, a)| (p, v, a))
        .to_vec();

    for iter in 0..10_000 {
        for (p, v, a) in parsed.iter_mut() {
            for i in 0..3 {
                v[i] += a[i];
                p[i] += v[i];
            }
        }
    }
    let min = parsed
        .iter()
        .enumerate()
        .min_by_key(|(_, (p, _, _))| manhattan_3d_i((p[0], p[1], p[2]), (0, 0, 0)))
        .unwrap();
    pv!(min.0);
}
