use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");
    // let input = ;

    let mut connectors = HashMap::new();
    for l in input.lines() {
        let v = l.split('/').map(parse_u).to_vec();
        connectors
            .entry(v[0])
            .or_insert_with(HashSet::new)
            .insert(v[1]);
        connectors
            .entry(v[1])
            .or_insert_with(HashSet::new)
            .insert(v[0]);
    }

    fn recurse(current: usize, map: HashMap<usize, HashSet<usize>>) -> (usize, usize) {
        if let Some(other) = map.get(&current) {
            let mut max = (0, 0);
            for v in other {
                let mut map = map.clone();
                map.get_mut(&current).unwrap().remove(v);
                map.get_mut(v).unwrap().remove(&current);
                let (mut length, mut strength) = recurse(*v, map);
                strength += current + *v;
                length += 1;
                if length > max.0 || (length == max.0 && strength > max.1) {
                    max = (length, strength);
                }
            }
            max
        } else {
            (0, 0)
        }
    }
    let result = recurse(0, connectors);
    pv!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");
    // let input = ;

    let mut connectors = HashMap::new();
    for l in input.lines() {
        let v = l.split('/').map(parse_u).to_vec();
        connectors
            .entry(v[0])
            .or_insert_with(HashSet::new)
            .insert(v[1]);
        connectors
            .entry(v[1])
            .or_insert_with(HashSet::new)
            .insert(v[0]);
    }

    fn recurse(current: usize, map: HashMap<usize, HashSet<usize>>) -> usize {
        if let Some(other) = map.get(&current) {
            let mut max = 0;
            for v in other {
                let mut map = map.clone();
                map.get_mut(&current).unwrap().remove(v);
                map.get_mut(v).unwrap().remove(&current);
                let strength = current + *v + recurse(*v, map);
                max = max.max(strength);
            }
            max
        } else {
            0
        }
    }
    let result = recurse(0, connectors);
    pv!(result);
}
