use regex::Regex;
use std::fs::File;
use std::io::Read;

// difficulty: 1

fn main() {
    //let mut file = File::open("inputs/day3.test").unwrap();
    let mut file = File::open("inputs/day3.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

fn part_one(data: String) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let result: i32 = re
        .captures_iter(&data)
        .map(|item| item[1].parse::<i32>().unwrap() * item[2].parse::<i32>().unwrap())
        .sum();

    println!("Result part 1: {result}");
}

#[derive(Copy, Clone)]
enum State {
    Do,
    Dont,
}

fn part_two(data: String) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let (_, result) = re.captures_iter(&data)
        .fold((State::Do, 0), |(state, x), item| match (&item[0], state) {
            ("don't()", _) => (State::Dont, x),
            ("do()", _) => (State::Do, x),
            (_, State::Do) => (state, x + (item[1].parse::<i32>().unwrap() * item[2].parse::<i32>().unwrap())),
            (_, State::Dont) => (state, x),
        });

    println!("Result part 2: {result}");
}
