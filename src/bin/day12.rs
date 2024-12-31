use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::Read;

// difficulty: 3

fn main() {
    //let mut file = File::open("inputs/day12.test").unwrap();
    let mut file = File::open("inputs/day12.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}
type Grid = Vec<Vec<char>>;

#[derive(Default, Debug)]
struct Region {
    tiles: HashSet<(i32, i32)>,
    area: u32,
    perimeter: u32,
    sides: u32,
}

fn search_region(grid: &Grid, seen: &mut HashSet<(usize, usize)>, pos: (usize, usize)) -> Region {
    let mut region = Region::default();
    let mut queue = VecDeque::new();
    let plant = grid[pos.0][pos.1];

    queue.push_back(pos);
    while let Some((i, j)) = queue.pop_back() {
        if seen.contains(&(i, j)) || grid[i][j] != plant {
            continue;
        }
        seen.insert((i, j));
        region.tiles.insert((i as i32, j as i32));

        region.area += 1;

        // borders
        if i == 0 || i == grid.len() - 1 {
            region.perimeter += 1;
        }
        if j == 0 || j == grid[0].len() - 1 {
            region.perimeter += 1;
        }

        // down
        if i < grid.len() - 1 && grid[i + 1][j] != plant {
            region.perimeter += 1;
        }

        // up
        if i > 0 && grid[i - 1][j] != plant {
            region.perimeter += 1;
        }

        // right
        if j < grid[0].len() - 1 && grid[i][j + 1] != plant {
            region.perimeter += 1;
        }

        // left
        if j > 0 && grid[i][j - 1] != plant {
            region.perimeter += 1;
        }

        if i < grid.len() - 1 {
            queue.push_back((i + 1, j));
        }
        if i > 0 {
            queue.push_back((i - 1, j));
        }
        if j < grid[0].len() - 1 {
            queue.push_back((i, j + 1));
        }
        if j > 0 {
            queue.push_back((i, j - 1));
        }
    }
    // sides
    for &(i, j) in region.tiles.iter() {
        if !region.tiles.contains(&(i - 1, j)) && !region.tiles.contains(&(i, j + 1)) {
            region.sides += 1
        }
        if !region.tiles.contains(&(i - 1, j)) && !region.tiles.contains(&(i, j - 1)) {
            region.sides += 1
        }
        if !region.tiles.contains(&(i + 1, j)) && !region.tiles.contains(&(i, j - 1)) {
            region.sides += 1
        }
        if !region.tiles.contains(&(i + 1, j)) && !region.tiles.contains(&(i, j + 1)) {
            region.sides += 1
        }

        if region.tiles.contains(&(i + 1, j))
            && region.tiles.contains(&(i, j + 1))
            && !region.tiles.contains(&(i + 1, j + 1))
        {
            region.sides += 1
        }
        if region.tiles.contains(&(i + 1, j))
            && region.tiles.contains(&(i, j - 1))
            && !region.tiles.contains(&(i + 1, j - 1))
        {
            region.sides += 1
        }
        if region.tiles.contains(&(i - 1, j))
            && region.tiles.contains(&(i, j - 1))
            && !region.tiles.contains(&(i - 1, j - 1))
        {
            region.sides += 1
        }
        if region.tiles.contains(&(i - 1, j))
            && region.tiles.contains(&(i, j + 1))
            && !region.tiles.contains(&(i - 1, j + 1))
        {
            region.sides += 1
        }
    }

    region
}

fn part_one(data: String) {
    let grid: Vec<_> = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut garden = Vec::new();
    let mut seen = HashSet::new();
    for (i, line) in grid.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            if seen.contains(&(i, j)) {
                continue;
            }
            let region = search_region(&grid, &mut seen, (i, j));
            garden.push(region);
        }
    }

    let result = garden
        .iter()
        .map(|region| region.perimeter * region.area)
        .sum::<u32>();

    println!("Result part 1: {result}");
}

fn part_two(data: String) {
    let grid: Vec<_> = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut garden = Vec::new();
    let mut seen = HashSet::new();
    for (i, line) in grid.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            if seen.contains(&(i, j)) {
                continue;
            }
            let region = search_region(&grid, &mut seen, (i, j));
            garden.push(region);
        }
    }

    let result = garden
        .iter()
        .map(|region| region.sides * region.area)
        .sum::<u32>();

    println!("Result part 2: {result}");
}
