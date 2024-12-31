use cached::proc_macro::cached;
use itertools::{iproduct, Itertools};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

// difficulty: 5

fn main() {
    //let mut file = File::open("inputs/day21.test").unwrap();
    let mut file = File::open("inputs/day21.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

enum Dir {
    Right,
    Left,
    Up,
    Down,
}
impl Dir {
    const ALL_DIRS: [Dir; 4] = [Dir::Right, Dir::Left, Dir::Up, Dir::Down];

    fn delta_index(&self) -> (i32, i32) {
        match self {
            Dir::Right => (0, 1),
            Dir::Left => (0, -1),
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
        }
    }
    fn to_str(&self) -> &str {
        match self {
            Dir::Right => ">",
            Dir::Left => "<",
            Dir::Up => "^",
            Dir::Down => "v",
        }
    }
}

trait Keypad {
    const KEYPAD: &'static [(i32, i32, char)];

    fn get_value((x, y): (i32, i32)) -> Option<char> {
        Self::KEYPAD
            .iter()
            .find(|&&(px, py, _)| px == x && py == y)
            .map(|&(_, _, c)| c)
    }

    fn calculate_paths() -> HashMap<(char, char), Vec<String>> {
        let mut map = HashMap::new();
        for (start, end) in iproduct!(Self::KEYPAD.iter(), Self::KEYPAD.iter()) {
            let start_pos = (start.0, start.1);
            let end_pos = (end.0, end.1);

            let mut paths: Vec<String> = Vec::new();

            let mut queue = VecDeque::new();
            queue.push_back((0, start_pos, String::new()));
            while let Some((count, pos, dirs)) = queue.pop_front() {
                if pos == end_pos {
                    if paths.is_empty() || paths[0].len() >= count {
                        paths.push(dirs.clone() + "A");
                        continue;
                    }
                    break;
                }
                for dir in Dir::ALL_DIRS {
                    let (di, dj) = dir.delta_index();
                    let next_pos = (pos.0 + di, pos.1 + dj);
                    if Self::get_value(next_pos).is_some() {
                        queue.push_back((count + 1, next_pos, dirs.clone() + dir.to_str()));
                    }
                }
            }
            map.insert((start.2, end.2), paths);
        }
        map
    }
}

struct NumKeypad;
impl Keypad for NumKeypad {
    const KEYPAD: &'static [(i32, i32, char)] = &[
        (3, 2, 'A'), // start pos
        (0, 0, '7'),
        (0, 1, '8'),
        (0, 2, '9'),
        (1, 0, '4'),
        (1, 1, '5'),
        (1, 2, '6'),
        (2, 0, '1'),
        (2, 1, '2'),
        (2, 2, '3'),
        (3, 1, '0'),
    ];
}

struct DirKeypad;
impl Keypad for DirKeypad {
    const KEYPAD: &'static [(i32, i32, char)] = &[
        (0, 2, 'A'), // start pos
        (0, 1, '^'),
        (1, 0, '<'),
        (1, 1, 'v'),
        (1, 2, '>'),
    ];
}

#[cached]
fn compute_length(targets: String, depth: u8, numeric: bool) -> usize {
    let paths = if numeric {
        NumKeypad::calculate_paths()
    } else {
        DirKeypad::calculate_paths()
    };
    ("A".to_string() + &targets)
        .chars()
        .tuple_windows()
        .map(|(x, y)| {
            let shortest_paths = paths.get(&(x, y)).unwrap();
            match depth {
                1 => shortest_paths[0].len(),
                _ => shortest_paths
                    .iter()
                    .cloned()
                    .map(|path| compute_length(path, depth - 1, false))
                    .min()
                    .unwrap(),
            }
        })
        .sum()
}

fn part_one(data: String) {
    let result: usize = data
        .lines()
        .map(|line| {
            compute_length(line.to_string(), 3, true)
                * line.trim_end_matches('A').parse::<usize>().unwrap()
        })
        .sum();

    println!("Result part 1: {result}");
}

fn part_two(data: String) {
    let result: usize = data
        .lines()
        .map(|line| {
            compute_length(line.to_string(), 26, true)
                * line.trim_end_matches('A').parse::<usize>().unwrap()
        })
        .sum();

    println!("Result part 2: {result}");
}
