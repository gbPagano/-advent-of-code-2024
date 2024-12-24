use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::Read;

// difficulty: 2

fn main() {
    //let mut file = File::open("inputs/day18.test").unwrap();
    let mut file = File::open("inputs/day18.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

const GRID_SIZE: usize = 71;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}
impl Dir {
    fn delta_index(&self) -> (i32, i32) {
        match self {
            Dir::Right => (0, 1),
            Dir::Left => (0, -1),
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
        }
    }
}

struct Grid([[char; GRID_SIZE]; GRID_SIZE]);
impl Grid {
    #[allow(dead_code)]
    fn print(&self) {
        for line in &self.0 {
            println!("{}", line.iter().collect::<String>());
        }
    }

    fn size(&self) -> (i32, i32) {
        (self.0.len() as i32, self.0[0].len() as i32)
    }

    fn get_possible_moves(&self, (i, j): (i32, i32)) -> Vec<(i32, i32)> {
        let mut result = Vec::new();
        for dir in &[Dir::Right, Dir::Left, Dir::Up, Dir::Down] {
            let (di, dj) = dir.delta_index();
            let (ni, nj) = (i + di, j + dj);
            let (mi, mj) = self.size();

            if ni < 0 || ni >= mi || nj < 0 || nj >= mj {
                continue;
            }
            if self.0[ni as usize][nj as usize] != '#' {
                result.push((ni, nj));
            }
        }
        result
    }
}

fn part_one(data: String) {
    let bytes = data
        .lines()
        .filter_map(|line| line.split_once(","))
        .map(|(a, b)| (b.parse::<usize>().unwrap(), a.parse::<usize>().unwrap()))
        .take(1024);

    let mut grid = Grid([['.'; GRID_SIZE]; GRID_SIZE]);
    for (i, j) in bytes {
        grid.0[i][j] = '#';
    }

    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), 0));
    while let Some((pos, steps)) = queue.pop_front() {
        if pos == (GRID_SIZE as i32 - 1, GRID_SIZE as i32 - 1) {
            println!("Result part 1: {steps}");
            break;
        }
        seen.insert(pos);

        for next_pos in grid.get_possible_moves(pos) {
            if !seen.contains(&next_pos) {
                queue.push_back((next_pos, steps + 1));
            }
            seen.insert(next_pos);
        }
    }
}

fn part_two(data: String) {
    let mut bytes = data
        .lines()
        .filter_map(|line| line.split_once(","))
        .map(|(a, b)| (b.parse::<usize>().unwrap(), a.parse::<usize>().unwrap()));

    let mut grid = Grid([['.'; GRID_SIZE]; GRID_SIZE]);
    for _ in 0..1024 {
        let (i, j) = bytes.next().unwrap();
        grid.0[i][j] = '#';
    }

    'bytes_loop: while let Some((x, y)) = bytes.next() {
        grid.0[x][y] = '#';

        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(((0, 0), 0));
        while let Some((pos, steps)) = queue.pop_front() {
            if pos == (GRID_SIZE as i32 - 1, GRID_SIZE as i32 - 1) {
                continue 'bytes_loop;
            }
            seen.insert(pos);

            for next_pos in grid.get_possible_moves(pos) {
                if !seen.contains(&next_pos) {
                    queue.push_back((next_pos, steps + 1));
                }
                seen.insert(next_pos);
            }
        }
                
        println!("Result part 2: {y},{x}");
        break;
    }
}
