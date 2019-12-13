use std::fs::read_to_string;

use std::collections::HashMap;

fn parse(input: &str) -> HashMap<&str, &str> {
    let mut map: HashMap<&str, &str> = HashMap::new();
    let orbs = input
        .lines()
        .map(|s| s.split(")").collect())
        .collect::<Vec<Vec<&str>>>();
    for o in orbs {
        map.entry(o[1]).or_insert(o[0]);
    }
    map
}

fn main() {
    let input = read_to_string("input").expect("failed to read input file");
    let map = parse(&input);

    println!(
        "Solution Part 1: {}",
        map.iter()
            .map(|o| get_chain(o.1, &map).len())
            .sum::<usize>()
            + map.len()
    );

    let overlap = get_first_overlap("YOU", "SAN", &map).unwrap();

    let get_overlap_idx = |o| {
        get_chain(o, &map)
            .iter()
            .position(|x| x == &overlap)
            .expect("Couldn't find overlap in path")
    };

    let idx_overlap_you = get_overlap_idx("YOU");
    let idx_overlap_san = get_overlap_idx("SAN");
    println!("Solution Part 2: {}", idx_overlap_san + idx_overlap_you - 2);
}

fn get_first_overlap<'a>(a: &'a str, b: &'a str, map: &HashMap<&str, &'a str>) -> Option<&'a str> {
    let b_chain = get_chain(b, &map);
    for p in get_chain(a, &map).iter() {
        match b_chain.iter().find(|s| &p == s) {
            Some(x) => return Some(x),
            None => (),
        };
    }
    None
}

fn get_chain<'a>(o: &'a str, map: &HashMap<&str, &'a str>) -> Vec<&'a str> {
    let mut current = o;
    let mut chain = Vec::new();
    while current != "COM" {
        chain.push(current);
        current = map.get(current).expect("Cannot find object in map")
    }
    chain
}
