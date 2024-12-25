use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

// difficulty: 4

fn main() {
    //let mut file = File::open("inputs/day20.test").unwrap();
    let mut file = File::open("inputs/day20.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

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

struct Grid(Vec<Vec<char>>);
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

    fn find_pos(&self, item: &char) -> (i32, i32) {
        self.0
            .iter()
            .enumerate()
            .find_map(|(i, line)| {
                line.iter()
                    .position(|x| x == item)
                    .map(|j| (i as i32, j as i32))
            })
            .unwrap()
    }
}

fn calculate_paths(grid: &Grid, start: (i32, i32), end: (i32, i32)) -> HashMap<(i32, i32), i32> {
    let mut pos = end;
    let mut picoseconds = 0;
    let mut seen = HashSet::new();
    let mut path: HashMap<(i32, i32), i32> = HashMap::new();
    loop {
        seen.insert(pos);
        path.insert(pos, picoseconds);
        if pos == start {
            break;
        }
        let next_positions: Vec<_> = grid.get_possible_moves(pos);
        let next_pos: Vec<_> = next_positions
            .iter()
            .filter(|&m| !seen.contains(m))
            .collect();

        // assumes that there is only one possible path, without multiple curves
        assert_eq!(next_pos.len(), 1);

        pos = *next_pos[0];
        picoseconds += 1;
    }
    path
}

fn part_one(data: String) {
    let grid = Grid(
        data.lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect(),
    );
    let start = grid.find_pos(&'S');
    let end = grid.find_pos(&'E');

    let path = calculate_paths(&grid, start, end);

    let mut pos = start;
    let mut picoseconds = 0;
    let mut seen = HashSet::new();
    let mut result = 0;
    loop {
        seen.insert(pos);
        if pos == end {
            break;
        }
        for (di, dj) in &[(0, 2), (0, -2), (2, 0), (-2, 0)] {
            let cheat_pos = (pos.0 + di, pos.1 + dj);
            if let Some(val) = path.get(&cheat_pos) {
                let cheat_result = picoseconds + val + 2;
                let saved_picoconds = path.get(&start).unwrap() - cheat_result;
                if saved_picoconds > 0 && saved_picoconds >= 100 {
                    result += 1;
                }
            }
        }

        // continue base path loop
        let next_positions: Vec<_> = grid.get_possible_moves(pos);
        let next_pos = next_positions
            .iter()
            .filter(|&m| !seen.contains(m))
            .next()
            .unwrap();

        pos = *next_pos;
        picoseconds += 1;
    }
    println!("Result part 1: {result}");
}

fn part_two(data: String) {
    let grid = Grid(
        data.lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect(),
    );
    let start = grid.find_pos(&'S');
    let end = grid.find_pos(&'E');

    let path = calculate_paths(&grid, start, end);
    let deltas = get_possible_delta_positions(20);

    let mut pos = start;
    let mut picoseconds = 0;
    let mut seen = HashSet::new();
    let mut result = 0;
    loop {
        seen.insert(pos);
        if pos == end {
            break;
        }
        for (di, dj) in &deltas {
            let cheat_pos = (pos.0 + di, pos.1 + dj);
            if let Some(val) = path.get(&cheat_pos) {
                let cheat_result = picoseconds + val + di.abs() + dj.abs();
                let saved_picoconds = path.get(&start).unwrap() - cheat_result;
                if saved_picoconds > 0
                    && saved_picoconds >= 100
                {
                    result += 1;
                }
            }
        }

        // continue base path loop
        let next_positions: Vec<_> = grid.get_possible_moves(pos);
        let next_pos = next_positions
            .iter()
            .filter(|&m| !seen.contains(m))
            .next()
            .unwrap();

        pos = *next_pos;
        picoseconds += 1;
    }

    println!("Result part 2: {result}");
}

fn get_possible_delta_positions(steps: i32) -> HashSet<(i32, i32)> {
    let mut possible_deltas = HashSet::new();
    possible_deltas.insert((0, 0));
    for _ in 1..=steps {
        let mut new_deltas = Vec::new();
        for (di, dj) in &possible_deltas {
            new_deltas.extend([(di + 1, *dj), (di - 1, *dj), (*di, dj + 1), (*di, dj - 1)]);
        }
        possible_deltas.extend(new_deltas);
    }
    possible_deltas
}
