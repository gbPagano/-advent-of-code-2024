use std::fs::File;
use std::io::Read;

// difficulty: 2

fn main() {
    //let mut file = File::open("inputs/day25.test").unwrap();
    let mut file = File::open("inputs/day25.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data);
}

fn get_heights(grid: &Vec<Vec<char>>) -> Vec<usize> {
    grid.iter()
        .map(|line| line.iter().filter(|&&x| x == '#').count() - 1)
        .collect()
}

fn part_one(data: String) {
    let grids: Vec<Vec<Vec<_>>> = data
        .split("\n\n")
        .map(|item| {
            item.lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .map(|matrix| {
            (0..matrix[0].len())
                .map(|i| matrix.iter().map(|row| row[i]).collect())
                .collect()
        })
        .collect();
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for grid in grids {
        if grid[0][0] == '#' {
            locks.push(get_heights(&grid));
        } else {
            keys.push(get_heights(&grid));
        }
    }
    
    let mut overlaps = 0;
    for l in &locks {
        for k in &keys {
           overlaps += !l.iter().zip(k).any(|(x, y)| x + y > 5) as u32;
        }
    
    }

    println!("Result part 1: {overlaps}");
}
