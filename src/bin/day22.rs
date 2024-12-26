use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

// difficulty: 4

fn main() {
    //let mut file = File::open("inputs/day22.test").unwrap();
    let mut file = File::open("inputs/day22.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

fn next_secret(mut num: u64) -> u64 {
    const MAGIC_NUMBER: u64 = 16777216;
    num = (num ^ num << 6) & (MAGIC_NUMBER - 1);
    num = (num ^ num >> 5) & (MAGIC_NUMBER - 1);
    num = (num ^ num << 11) & (MAGIC_NUMBER - 1);
    num
}

fn n_next_secret(mut num: u64, n: u64) -> u64 {
    for _ in 0..n {
        num = next_secret(num);
    }
    num
}

fn part_one(data: String) {
    let nums = data.lines().map(|line| line.parse::<u64>().unwrap());

    let result: u64 = nums.map(|n| n_next_secret(n, 2000)).sum();
    println!("Result part 1: {result}");
}

fn get_sell_prices(mut num: u64, n: u16) -> Vec<u64> {
    let mut sell_prices = Vec::new();
    for _ in 0..n {
        num = next_secret(num);
        sell_prices.push(num % 10);
    }

    sell_prices
}

fn part_two(data: String) {
    let nums = data.lines().map(|line| line.parse::<u64>().unwrap());

    let mut seq_total_count = HashMap::new();
    for n in nums {
        let sell_prices = get_sell_prices(n, 2000);
        let mut seen = HashSet::new();
        for i in 0..sell_prices.len() - 4 {
            let (a, b, c, d, e) = (
                sell_prices[i] as i32,
                sell_prices[i + 1] as i32,
                sell_prices[i + 2] as i32,
                sell_prices[i + 3] as i32,
                sell_prices[i + 4] as i32,
            );
            let seq = (b - a, c - b, d - c, e - d);
            if seen.contains(&seq) {
                continue;
            };
            seen.insert(seq);
            seq_total_count
                .entry(seq)
                .and_modify(|count| *count += e)
                .or_insert(e);
        }
    }

    let result = seq_total_count.into_values().max().unwrap();
    println!("Result part 2: {result}");
}
