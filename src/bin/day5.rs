use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

// difficulty: 4

fn main() {
    //let mut file = File::open("inputs/day5.test").unwrap();
    let mut file = File::open("inputs/day5.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

#[derive(Debug)]
struct Rule {
    before: i32,
    after: i32,
}

fn part_one(data: String) {
    let (rules, pages) = data.split_once("\n\n").unwrap();
    let rules: Vec<Rule> = rules
        .lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(a, b)| Rule {
            before: a.parse().unwrap(),
            after: b.parse().unwrap(),
        })
        .collect();
    let pages: Vec<_> = pages
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut after_hashmap: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules.iter() {
        after_hashmap
            .entry(rule.before)
            .or_insert_with(Vec::new)
            .push(rule.after);
    }

    let result: i32 = pages
        .iter()
        .map(|page| {
            (
                page,
                page.iter()
                    .enumerate()
                    .filter(|(i, &num)| {
                        !page[i + 1..]
                            .iter()
                            .any(|x| !after_hashmap.entry(num).or_default().contains(x))
                    })
                    .count(),
            )
        })
        .filter_map(|(page, count)| {
            if count == page.len() {
                Some(page[page.len() / 2])
            } else {
                None
            }
        })
        .sum();

    println!("Result part 1: {result}");
}

fn part_two(data: String) {
    let (rules, pages) = data.split_once("\n\n").unwrap();
    let rules: Vec<Rule> = rules
        .lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(a, b)| Rule {
            before: a.parse().unwrap(),
            after: b.parse().unwrap(),
        })
        .collect();
    let pages: Vec<_> = pages
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut after_hashmap: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules.iter() {
        after_hashmap
            .entry(rule.before)
            .or_insert_with(Vec::new)
            .push(rule.after);
    }

    let mut wrong_pages: Vec<_> = pages
        .iter()
        .map(|page| {
            (
                page,
                page.iter()
                    .enumerate()
                    .filter(|(i, &num)| {
                        !page[i + 1..]
                            .iter()
                            .any(|x| !after_hashmap.entry(num).or_default().contains(x))
                    })
                    .count(),
            )
        })
        .filter_map(|(page, count)| {
            if count != page.len() {
                Some(page.clone())
            } else {
                None
            }
        })
        .collect();

    // ordering
    wrong_pages.iter_mut().for_each(|page| {
        page.sort_by(|a, b| {
            if after_hashmap.entry(*b).or_default().contains(a) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
    });
    
    let result: i32 = wrong_pages.iter().map(|page| page[page.len() / 2]).sum();
    println!("Result part 2: {result}");
}
