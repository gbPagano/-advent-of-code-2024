use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

// difficulty: 4

fn main() {
    //let mut file = File::open("inputs/day6.test").unwrap();
    let mut file = File::open("inputs/day6.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}
impl Dir {
    fn next_indexes(
        &self,
        (i, j): (usize, usize),
        (max_i, max_j): (usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Dir::Up => {
                if i == 0 {
                    None
                } else {
                    Some((i - 1, j))
                }
            }
            Dir::Down => {
                if i + 1 >= max_i {
                    None
                } else {
                    Some((i + 1, j))
                }
            }
            Dir::Right => {
                if j + 1 >= max_j {
                    None
                } else {
                    Some((i, j + 1))
                }
            }
            Dir::Left => {
                if j == 0 {
                    None
                } else {
                    Some((i, j - 1))
                }
            }
        }
    }

    fn next_dir(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Right => Dir::Down,
            Dir::Left => Dir::Up,
        }
    }
}

fn part_one(data: String) {
    let grid: Vec<_> = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let (num_lines, num_cols) = (grid.len(), grid[0].len());

    let mut curr_pos: (usize, usize) = grid
        .iter()
        .flatten()
        .enumerate()
        .filter_map(|(idx, c)| if *c == '^' { Some(idx) } else { None })
        .next()
        .map(|idx| (idx / num_cols, idx % num_cols))
        .unwrap();

    let mut seen_positions: HashSet<(usize, usize)> = HashSet::new();

    let mut curr_dir = Dir::Up;
    loop {
        seen_positions.insert(curr_pos);

        let (ni, nj) = match curr_dir.next_indexes(curr_pos, (num_lines, num_cols)) {
            Some(val) => val,
            None => break,
        };

        match grid[ni][nj] {
            '#' => {
                curr_dir = curr_dir.next_dir();
            }
            _ => {
                curr_pos = (ni, nj);
            }
        }
    }

    let result = seen_positions.len();
    println!("Result part 1: {result}");
}

fn part_two(data: String) {
    let mut grid: Vec<_> = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let (num_lines, num_cols) = (grid.len(), grid[0].len());

    let start_pos: (usize, usize) = grid
        .iter()
        .flatten()
        .enumerate()
        .filter_map(|(idx, c)| if *c == '^' { Some(idx) } else { None })
        .next()
        .map(|idx| (idx / num_cols, idx % num_cols))
        .unwrap();

    let mut seen_positions: HashSet<((usize, usize), Dir)> = HashSet::new();
    let mut seen_loop_pos: HashSet<(usize, usize)> = HashSet::new();
    let mut curr_pos = start_pos;
    let mut curr_dir = Dir::Up;
    let mut result = 0;
    loop {
        seen_positions.insert((curr_pos, curr_dir));

        let (ni, nj) = match curr_dir.next_indexes(curr_pos, (num_lines, num_cols)) {
            Some(val) => val,
            None => break,
        };

        match grid[ni][nj] {
            '#' => {
                curr_dir = curr_dir.next_dir();
            }
            c => {
                if !seen_loop_pos.contains(&(ni, nj)) {
                    grid[ni][nj] = '#';
                    if is_loop(&grid, curr_pos, curr_dir) {
                        result += 1;
                    }
                    seen_loop_pos.insert((ni, nj));
                }
                grid[ni][nj] = c;

                curr_pos = (ni, nj);
            }
        }
    }
    println!("Result part 2: {result}");
}

fn is_loop(grid: &[Vec<char>], start_pos: (usize, usize), start_dir: Dir) -> bool {
    let (num_lines, num_cols) = (grid.len(), grid[0].len());
    let (mut curr_pos, mut curr_dir) = (start_pos, start_dir);
    let mut seen = HashSet::new();
    loop {
        seen.insert((curr_pos, curr_dir));

        let (ni, nj) = match curr_dir.next_indexes(curr_pos, (num_lines, num_cols)) {
            Some(val) => val,
            None => return false,
        };

        match grid[ni][nj] {
            '#' => {
                curr_dir = curr_dir.next_dir();
            }
            _ => {
                curr_pos = (ni, nj);
            }
        }

        if seen.contains(&(curr_pos, curr_dir)) {
            return true;
        }
    }
}
