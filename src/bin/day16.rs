use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::Read;
use std::ops::Neg;

// difficulty: 3

fn main() {
    //let mut file = File::open("inputs/day16.test").unwrap();
    let mut file = File::open("inputs/day16.input").unwrap();
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
impl Neg for Dir {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Dir::Right => Dir::Left,
            Dir::Left => Dir::Right,
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
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

    fn get_possible_moves(&self, (i, j): (i32, i32)) -> Vec<((i32, i32), Dir)> {
        let mut result = Vec::new();
        for dir in &[Dir::Right, Dir::Left, Dir::Up, Dir::Down] {
            let (di, dj) = dir.delta_index();
            let (ni, nj) = (i + di, j + dj);
            let (mi, mj) = self.size();

            if ni < 0 || ni >= mi || nj < 0 || nj >= mj {
                continue;
            }
            if self.0[ni as usize][nj as usize] != '#' {
                result.push(((ni, nj), *dir));
            }
        }
        result
    }
}

fn part_one(data: String) {
    let grid = Grid(
        data.lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
    let start_pos = grid
        .0
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .position(|&x| x == 'S')
                .map(|j| (i as i32, j as i32))
        })
        .unwrap();
    let end_pos = grid
        .0
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .position(|&x| x == 'E')
                .map(|j| (i as i32, j as i32))
        })
        .unwrap();
    let start_dir = Dir::Right;

    let mut heap_queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    heap_queue.push((Reverse(0), start_pos, start_dir));

    while let Some((Reverse(points), pos, dir)) = heap_queue.pop() {
        if pos == end_pos {
            println!("Result part 1: {points}");
            break;
        }

        seen.insert((pos, dir));
        for (npos, ndir) in grid.get_possible_moves(pos) {
            if seen.contains(&(npos, ndir)) || ndir == -dir {
                continue;
            }
            let delta_points = if ndir == dir { 1 } else { 1001 };
            heap_queue.push((Reverse(points + delta_points), npos, ndir));
        }
    }
}

fn part_two(data: String) {
    let grid = Grid(
        data.lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
    let start_pos = grid
        .0
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .position(|&x| x == 'S')
                .map(|j| (i as i32, j as i32))
        })
        .unwrap();
    let end_pos = grid
        .0
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .position(|&x| x == 'E')
                .map(|j| (i as i32, j as i32))
        })
        .unwrap();
    let start_dir = Dir::Right;

    let mut heap_queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    let mut best_seen: HashSet<(i32, i32)> = HashSet::new();
    heap_queue.push((Reverse(0), start_pos, start_dir, vec![start_pos]));

    let mut min_score = u32::MAX;
    while let Some((Reverse(points), pos, dir, tiles)) = heap_queue.pop() {
        if pos == end_pos && points <= min_score {
            min_score = points;
            best_seen.extend(tiles.iter());
        } else if min_score < u32::MAX && points > min_score {
            break;
        }

        seen.insert((pos, dir));
        for (npos, ndir) in grid.get_possible_moves(pos) {
            if seen.contains(&(npos, ndir)) || ndir == -dir {
                continue;
            }
            let delta_points = if ndir == dir { 1 } else { 1001 };
            let mut ntiles = tiles.clone();
            ntiles.push(npos);
            heap_queue.push((Reverse(points + delta_points), npos, ndir, ntiles));
        }
    }

    println!("Result part 2: {}", best_seen.len());
}
