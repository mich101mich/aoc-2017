use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");
    // let input = ;

    let parsed = input
        .lines()
        .map(|l| {
            let mut s = l.split(" <-> ");
            let key = parse_u(s.next().unwrap());
            let values = s.next().unwrap().split(", ").map(parse_u).to_vec();
            (key, values)
        })
        .collect::<HashMap<_, _>>();

    let mut all_nodes = parsed.keys().copied().to_set();

    let mut group_count = 0;
    while !all_nodes.is_empty() {
        let start = *all_nodes.iter().next().unwrap();
        let reachable = dijkstra_search(
            |n| parsed[&n].iter().copied(),
            |_, _| 1,
            |_| true,
            start,
            &all_nodes.iter().copied().to_vec(),
        );
        for n in reachable.keys() {
            all_nodes.remove(n);
        }
        group_count += 1;
    }
    pv!(group_count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");
    // let input = ;

    let parsed = input
        .lines()
        .map(|l| {
            let mut s = l.split(" <-> ");
            let key = parse_u(s.next().unwrap());
            let values = s.next().unwrap().split(", ").map(parse_u).to_vec();
            (key, values)
        })
        .collect::<HashMap<_, _>>();

    let all_nodes = parsed.keys().copied().to_vec();
    let reachable = dijkstra_search(
        |n| parsed[&n].iter().copied(),
        |_, _| 1,
        |_| true,
        0usize,
        &all_nodes,
    );
    pv!(reachable.len());
}
