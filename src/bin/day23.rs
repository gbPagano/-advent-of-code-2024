use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

// difficulty: 4

fn main() {
    //let mut file = File::open("inputs/day23.test").unwrap();
    let mut file = File::open("inputs/day23.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}
type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn part_one(data: String) {
    let mut graph: Graph = HashMap::new();
    data.lines()
        .map(|line| line.split_once("-").unwrap())
        .for_each(|(a, b)| {
            graph.entry(a).or_default().insert(b);
            graph.entry(b).or_default().insert(a);
        });

    let mut three_interconnected = HashSet::new();
    for (k, v) in graph.iter() {
        for pair in v.iter().combinations(2) {
            if graph.get(pair[0]).unwrap().contains(pair[1]) {
                let mut set = [k, pair[0], pair[1]];
                set.sort();
                three_interconnected.insert(set);
            }
        }
    }

    let mut result = 0;
    for set in three_interconnected {
        if set.iter().any(|k| k.starts_with('t')) {
            result += 1;
        }
    }

    println!("Result part 1: {result}");
}

fn part_two(data: String) {
    let mut graph: Graph = HashMap::new();
    data.lines()
        .map(|line| line.split_once("-").unwrap())
        .for_each(|(a, b)| {
            graph.entry(a).or_default().insert(b);
            graph.entry(b).or_default().insert(a);
        });

    let mut sets = HashSet::new();
    for node in graph.keys() {
        search(&graph, node, &mut vec![node], &mut sets);
    }

    let max_set = sets.iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap();
    let result = max_set.iter().join(",");

    println!("Result part 2: {result}");
}

fn search<'a>(
    graph: &'a Graph,
    node: &str,
    required: &mut Vec<&'a str>,
    sets: &mut HashSet<Vec<&'a str>>,
) {
    required.sort();
    if sets.contains(required) {
        return;
    }
    sets.insert(required.clone());

    for neighbour in graph.get(node).unwrap() {
        if required.contains(neighbour)
            || required
                .iter()
                .any(|n| !graph.get(neighbour).unwrap().contains(n))
        {
            continue;
        }

        required.push(neighbour);
        search(graph, neighbour, required, sets);
        required.retain(|item| item == neighbour);
    }
}
