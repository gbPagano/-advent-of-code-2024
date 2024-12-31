use std::fs::File;
use std::io::Read;

// difficulty: 1

fn main() {
    //let mut file = File::open("inputs/day1.test").unwrap();
    let mut file = File::open("inputs/day1.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

fn part_one(data: String) {
    let (mut first_items, mut second_items): (Vec<_>, Vec<_>) = data
        .trim()
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|items| {
            (
                items[0].parse::<i32>().unwrap(),
                items[1].parse::<i32>().unwrap(),
            )
        })
        .unzip();

    first_items.sort();
    second_items.sort();

    let res: i32 = first_items
        .iter()
        .zip(second_items)
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Result part 1: {res}");
}

fn part_two(data: String) {
    let (first_items, second_items): (Vec<_>, Vec<_>) = data
        .trim()
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|items| {
            (
                items[0].parse::<i32>().unwrap(),
                items[1].parse::<i32>().unwrap(),
            )
        })
        .unzip();

    let res: i32 = first_items
        .iter()
        .map(|item| item * second_items.iter().filter(|&x| x == item).count() as i32)
        .sum();

    println!("Result part 2: {res}");
}
