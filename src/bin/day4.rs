use std::fs::File;
use std::io::Read;

// difficulty: 2

fn main() {
    //let mut file = File::open("inputs/day4.test").unwrap();
    let mut file = File::open("inputs/day4.input").unwrap();
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

    let mut result = 0;
    for (i, line) in grid.iter().enumerate() {
        for (j, item) in line.iter().enumerate() {
            if *item != 'X' {
                continue;
            }
            // right
            if j <= line.len() - 4
                && grid[i][j + 1] == 'M'
                && grid[i][j + 2] == 'A'
                && grid[i][j + 3] == 'S'
            {
                result += 1;
            }
            // left
            if j >= 3 && grid[i][j - 1] == 'M' && grid[i][j - 2] == 'A' && grid[i][j - 3] == 'S' {
                result += 1;
            }
            // up
            if i >= 3 && grid[i - 1][j] == 'M' && grid[i - 2][j] == 'A' && grid[i - 3][j] == 'S' {
                result += 1;
            }
            // down
            if i <= grid.len() - 4
                && grid[i + 1][j] == 'M'
                && grid[i + 2][j] == 'A'
                && grid[i + 3][j] == 'S'
            {
                result += 1;
            }
            // upper right diagonal
            if j <= line.len() - 4
                && i >= 3
                && grid[i - 1][j + 1] == 'M'
                && grid[i - 2][j + 2] == 'A'
                && grid[i - 3][j + 3] == 'S'
            {
                result += 1;
            }
            // bottom right diagonal
            if j <= line.len() - 4
                && i <= grid.len() - 4
                && grid[i + 1][j + 1] == 'M'
                && grid[i + 2][j + 2] == 'A'
                && grid[i + 3][j + 3] == 'S'
            {
                result += 1;
            }
            // upper left diagonal
            if j >= 3
                && i >= 3
                && grid[i - 1][j - 1] == 'M'
                && grid[i - 2][j - 2] == 'A'
                && grid[i - 3][j - 3] == 'S'
            {
                result += 1;
            }
            // bottom left diagonal
            if j >= 3
                && i <= grid.len() - 4
                && grid[i + 1][j - 1] == 'M'
                && grid[i + 2][j - 2] == 'A'
                && grid[i + 3][j - 3] == 'S'
            {
                result += 1;
            }
        }
    }

    println!("Result part 1: {result}");
}

fn part_two(data: String) {
    let grid: Vec<_> = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut result = 0;
    for (i, line) in grid.iter().enumerate() {
        for (j, item) in line.iter().enumerate() {
            if *item != 'A' || i < 1 || j < 1 || i > line.len() - 2 || j > grid.len() - 2 {
                continue;
            }

            // m . s | m . s
            if grid[i - 1][j - 1] == 'M'
                && grid[i - 1][j + 1] == 'S'
                && grid[i + 1][j - 1] == 'M'
                && grid[i + 1][j + 1] == 'S'
            {
                result += 1;
            }
            // m . m | s . s
            if grid[i - 1][j - 1] == 'M'
                && grid[i - 1][j + 1] == 'M'
                && grid[i + 1][j - 1] == 'S'
                && grid[i + 1][j + 1] == 'S'
            {
                result += 1;
            }
            // s . m | s . m
            if grid[i - 1][j - 1] == 'S'
                && grid[i - 1][j + 1] == 'M'
                && grid[i + 1][j - 1] == 'S'
                && grid[i + 1][j + 1] == 'M'
            {
                result += 1;
            }
            // s . s | m . m
            if grid[i - 1][j - 1] == 'S'
                && grid[i - 1][j + 1] == 'S'
                && grid[i + 1][j - 1] == 'M'
                && grid[i + 1][j + 1] == 'M'
            {
                result += 1;
            }
        }
    }

    println!("Result part 2: {result}");
}
