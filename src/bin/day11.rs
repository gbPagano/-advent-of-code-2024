use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

// difficulty: 5

fn main() {
    //let mut file = File::open("inputs/day11.test").unwrap();
    let mut file = File::open("inputs/day11.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

fn part_one(data: String) {
    let mut stones: Vec<u64> = data
        .split_whitespace()
        .map(|item| item.parse::<u64>().unwrap())
        .collect();

    for _ in 0..25 {
        stones = blink_1(stones);
    }

    println!("Result part 1: {}", stones.len());
}

fn part_two(data: String) {
    let mut stones: HashMap<_, _> = data
        .split_whitespace()
        .map(|item| (item.parse::<u64>().unwrap(), 1))
        .collect();

    for _ in 0..75 {
        blink_2(&mut stones);
    }

    let result = stones.values().sum::<u64>();
    println!("Result part 2: {result}");
}

fn blink_2(stones: &mut HashMap<u64, u64>) {
    let mut new_stones = HashMap::new();

    for (&stone, val) in stones.iter_mut() {
        if stone == 0 {
            new_stones
                .entry(1)
                .and_modify(|v| *v += *val)
                .or_insert(*val);
            continue;
        }
        if has_even_digits(stone) {
            let (a, b) = split_number(stone);
            new_stones
                .entry(a)
                .and_modify(|v| *v += *val)
                .or_insert(*val);
            new_stones
                .entry(b)
                .and_modify(|v| *v += *val)
                .or_insert(*val);
            continue;
        }
        new_stones
            .entry(stone * 2024)
            .and_modify(|v| *v += *val)
            .or_insert(*val);
        *val = 0;
    }
    *stones = new_stones;
}

fn blink_1(stones: Vec<u64>) -> Vec<u64> {
    let mut new_stones = Vec::new();
    for stone in stones {
        if stone == 0 {
            new_stones.push(1);
            continue;
        }
        if has_even_digits(stone) {
            let (a, b) = split_number(stone);
            new_stones.push(a);
            new_stones.push(b);
            continue;
        }
        new_stones.push(stone * 2024);
    }
    new_stones
}

fn has_even_digits(num: u64) -> bool {
    let num_digits = (num as f64).log(10.0).floor() as u64 + 1;
    num_digits % 2 == 0
}

fn split_number(num: u64) -> (u64, u64) {
    let num_digits = (num as f64).log(10.0).floor() as u64 + 1;
    let half = num_digits / 2;

    let divisor = 10u64.pow((num_digits - half) as u32);

    let left = num / divisor;
    let right = num % divisor;

    (left, right)
}
