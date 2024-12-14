use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::Read;

// difficulty: 1

fn main() {
    //let mut file = File::open("inputs/day10.test").unwrap();
    let mut file = File::open("inputs/day10.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

type Grid = Vec<Vec<u32>>;

fn part_one(data: String) {
    let grid: Vec<_> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut result = 0;
    for (i, line) in grid.iter().enumerate() {
        for (j, tile) in line.iter().enumerate() {
            if *tile == 0 {
                result += score_trailheads((i, j), &grid);
            }
        }
    }
    println!("Result part 1: {result}");
}

fn part_two(data: String) {
    let grid: Vec<_> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut result = 0;
    for (i, line) in grid.iter().enumerate() {
        for (j, tile) in line.iter().enumerate() {
            if *tile == 0 {
                result += rating_trailheads((i, j), &grid);
            }
        }
    }
    println!("Result part 2: {result}");
}


fn score_trailheads(pos: (usize, usize), grid: &Grid) -> usize {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_front(pos);

    while let Some((i, j)) = queue.pop_front() {
        let curr = grid[i][j];
        if curr == 9 {
            seen.insert((i, j));
            continue;
        }

        if i + 1 < grid.len() && grid[i + 1][j] == curr + 1 {
            queue.push_front((i + 1, j));
        }

        if i > 0 && grid[i - 1][j] == curr + 1 {
            queue.push_front((i - 1, j));
        }

        if j + 1 < grid[0].len() && grid[i][j + 1] == curr + 1 {
            queue.push_front((i, j + 1));
        }

        if j > 0 && grid[i][j - 1] == curr + 1 {
            queue.push_front((i, j - 1));
        }
    }
    seen.len()
}
fn rating_trailheads(pos: (usize, usize), grid: &Grid) -> usize {
    let mut result = 0;
    let mut queue = VecDeque::new();
    queue.push_front(pos);

    while let Some((i, j)) = queue.pop_front() {
        let curr = grid[i][j];
        if curr == 9 {
            result += 1;
            continue;
        }

        if i + 1 < grid.len() && grid[i + 1][j] == curr + 1 {
            queue.push_front((i + 1, j));
        }

        if i > 0 && grid[i - 1][j] == curr + 1 {
            queue.push_front((i - 1, j));
        }

        if j + 1 < grid[0].len() && grid[i][j + 1] == curr + 1 {
            queue.push_front((i, j + 1));
        }

        if j > 0 && grid[i][j - 1] == curr + 1 {
            queue.push_front((i, j - 1));
        }
    }
    result
}
