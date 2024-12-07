use std::fs::File;
use std::io::Read;

// difficulty: 2

fn main() {
    //let mut file = File::open("inputs/day7.test").unwrap();
    let mut file = File::open("inputs/day7.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

fn part_one(data: String) {
    let data: Vec<_> = data
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(test, numbers)| {
            (
                test.parse::<usize>().unwrap(),
                numbers
                    .split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    
    let mut result = 0;
    for (target, numbers) in data {
        if is_valid(target, numbers[0], &numbers[1..]) {
            result += target;
        }
    }

    println!("Result part 1: {result}");
}

fn part_two(data: String) {
    let data: Vec<_> = data
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(test, numbers)| {
            (
                test.parse::<usize>().unwrap(),
                numbers
                    .split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    
    let mut result = 0;
    for (target, numbers) in data {
        if is_valid_p2(target, numbers[0], &numbers[1..]) {
            result += target;
        }
    }

    println!("Result part 2: {result}");
}

fn is_valid(target: usize, curr: usize, numbers: &[usize]) -> bool {
    if numbers.is_empty() {
        return curr == target;
    }

    is_valid(target, curr + numbers[0], &numbers[1..])
        || is_valid(target, curr * numbers[0], &numbers[1..])
}

fn is_valid_p2(target: usize, curr: usize, numbers: &[usize]) -> bool {
    if numbers.is_empty() {
        return curr == target;
    }

    is_valid_p2(target, curr + numbers[0], &numbers[1..])
        || is_valid_p2(target, curr * numbers[0], &numbers[1..])
        || is_valid_p2(target, concat(curr, numbers[0]), &numbers[1..])
}

fn concat(a: usize, b: usize) -> usize {
    (a.to_string() + &b.to_string()).parse::<usize>().unwrap()
}
