use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

// difficulty: 3

fn main() {
    //let mut file = File::open("inputs/day19.test").unwrap();
    let mut file = File::open("inputs/day19.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

fn is_valid<'a>(design: &'a str, patterns: &Vec<&str>, cache: &mut HashMap<&'a str, u64>) -> u64 {
    if let Some(val) = cache.get(&design) {
        return *val;
    }

    if design.is_empty() {
        return 1;
    }
    let mut result = 0;
    for p in patterns {
        if p.len() > design.len() {
            continue;
        };

        if **p == design[..p.len()] {
            result += is_valid(&design[p.len()..], patterns, cache);
        }
    }

    cache.insert(design, result);
    result
}

fn part_one(data: String) {
    let (patterns, designs) = data.split_once("\n\n").unwrap();
    let patterns: Vec<_> = patterns.split(", ").collect();
    let designs: Vec<_> = designs.lines().collect();

    let mut cache: HashMap<&str, u64> = HashMap::new();
    let result = designs
        .iter()
        .filter(|d| is_valid(d, &patterns, &mut cache) > 0)
        .count();
    println!("Result part 1: {result}");
}

fn part_two(data: String) {
    let (patterns, designs) = data.split_once("\n\n").unwrap();
    let patterns: Vec<_> = patterns.split(", ").collect();
    let designs: Vec<_> = designs.lines().collect();

    let mut cache: HashMap<&str, u64> = HashMap::new();
    let result: u64 = designs
        .iter()
        .map(|d| is_valid(d, &patterns, &mut cache))
        .sum();
    println!("Result part 2: {result}");
}
