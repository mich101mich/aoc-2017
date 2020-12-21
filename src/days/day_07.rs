use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");
    //     let input = "pbga (66)
    // xhth (57)
    // ebii (61)
    // havc (66)
    // ktlj (57)
    // fwft (72) -> ktlj, cntj, xhth
    // qoyq (66)
    // padx (45) -> pbga, havc, qoyq
    // tknk (41) -> ugml, padx, fwft
    // jptl (61)
    // ugml (68) -> gyxo, ebii, jptl
    // gyxo (61)
    // cntj (57)";

    let mut parents = HashMap::new();
    let mut children = HashMap::new();
    let mut disk_weight = HashMap::new();

    for line in input.lines() {
        let mut iter = line.split(' ');
        let name = iter.next().unwrap();
        let weight = iter.next().unwrap();

        let weight = isize::from_str(&weight[1..weight.len() - 1]).unwrap();
        disk_weight.insert(name, weight);

        if iter.next().is_some() {
            // arrow
            let child_disks = iter.map(|c| c.strip_suffix(',').unwrap_or(c)).to_vec();

            for &child in &child_disks {
                parents.insert(child, name);
            }

            children.insert(name, child_disks);
        }
    }
    let mut root = *parents.keys().next().unwrap();
    while parents.contains_key(root) {
        root = parents[root];
    }

    fn get_weight(name: &str, d: &HashMap<&str, isize>, c: &HashMap<&str, Vec<&str>>) -> isize {
        let mut me = d[name];
        if let Some(children) = c.get(name) {
            me += children
                .iter()
                .map(|&child| get_weight(child, d, c))
                .sum::<isize>();
        }
        me
    };

    let mut wrong_child = root;
    let mut done = false;

    loop {
        let my_children = &children[wrong_child];
        let mut child_weights = my_children
            .iter()
            .map(|c| (c, get_weight(c, &disk_weight, &children)))
            .to_vec();

        child_weights.sort_unstable_by_key(|(_, w)| *w);
        let correct = child_weights[child_weights.len() / 2].1;
        if let Some(next) = child_weights.iter().find(|(_, w)| *w != correct) {
            if done {
                let diff = correct - next.1;
                let result = disk_weight[next.0] + diff;
                pv!(result);
                break;
            }
            wrong_child = next.0;
        } else {
            wrong_child = parents[wrong_child];
            done = true;
        }
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");
    //     let input = "pbga (66)
    // xhth (57)
    // ebii (61)
    // havc (66)
    // ktlj (57)
    // fwft (72) -> ktlj, cntj, xhth
    // qoyq (66)
    // padx (45) -> pbga, havc, qoyq
    // tknk (41) -> ugml, padx, fwft
    // jptl (61)
    // ugml (68) -> gyxo, ebii, jptl
    // gyxo (61)
    // cntj (57)";

    let mut parents = HashMap::new();
    let mut disks = HashMap::new();

    for line in input.lines() {
        let mut iter = line.split(' ');
        let name = iter.next().unwrap();
        let weight = iter.next().unwrap();

        let weight = isize::from_str(&weight[1..weight.len() - 1]).unwrap();
        disks.insert(name, weight);

        if iter.next().is_some() {
            // arrow
            let children = iter.map(|c| c.strip_suffix(',').unwrap_or(c)).to_vec();

            for child in children {
                parents.insert(child, name);
            }
        }
    }
    let mut root = *parents.keys().next().unwrap();
    while parents.contains_key(root) {
        root = parents[root];
    }

    pv!(root);
}
