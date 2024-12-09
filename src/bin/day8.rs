use itertools::iproduct;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

// difficulty: 4

fn main() {
    //let mut file = File::open("inputs/day8.test").unwrap();
    let mut file = File::open("inputs/day8.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

fn part_one(data: String) {
    let grid: Vec<_> = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let (lines, columns) = (grid.len() as i32, grid[0].len() as i32);

    let mut seen = HashSet::new();
    let mut nodes = HashSet::new();
    for (i, j) in iproduct!(0..lines, 0..columns) {
        for (k, l) in iproduct!(0..lines, 0..columns) {
            let antenna_a = grid[i as usize][j as usize];
            let antenna_b = grid[k as usize][l as usize];
            if antenna_a != antenna_b
                || antenna_a == '.'
                || (i, j) == (k, l)
                || seen.contains(&(i, j, k, l))
            {
                continue;
            }
            seen.insert((i, j, k, l));
            seen.insert((k, l, i, j));

            let (di, dj) = ((k - i), (l - j));

            if i - di < lines && j - dj < columns && i - di >= 0 && j - dj >= 0 {
                nodes.insert((i - di, j - dj));
            }
            if k + di < lines && l + dj < columns && k + di >= 0 && l + dj >= 0 {
                nodes.insert((k + di, l + dj));
            }
        }
    }
    println!("Result part 1: {}", nodes.len());
}

fn part_two(data: String) {
    let grid: Vec<_> = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let (lines, columns) = (grid.len() as i32, grid[0].len() as i32);

    let mut seen = HashSet::new();
    let mut nodes = HashSet::new();
    for (i, j) in iproduct!(0..lines, 0..columns) {
        for (k, l) in iproduct!(0..lines, 0..columns) {
            let antenna_a = grid[i as usize][j as usize];
            let antenna_b = grid[k as usize][l as usize];
            if antenna_a != antenna_b
                || antenna_a == '.'
                || (i, j) == (k, l)
                || seen.contains(&(i, j, k, l))
            {
                continue;
            }
            seen.insert((i, j, k, l));
            seen.insert((k, l, i, j));

            let (di, dj) = ((k - i), (l - j));

            let (mut ni, mut nj) = (i - di, j - dj);
            while ni < lines && nj < columns && ni >= 0 && nj >= 0 {
                nodes.insert((ni, nj));
                (ni, nj) = (ni - di, nj - dj);
            }

            let (mut nk, mut nl) = (k - di, l - dj);
            while nk < lines && nl < columns && nk >= 0 && nl >= 0 {
                nodes.insert((nk, nl));
                (nk, nl) = (nk + di, nl + dj);
            }
        }
    }
    println!("Result part 2: {}", nodes.len());
}
