use std::fs::File;
use std::io::Read;

// difficulty: 3

fn main() {
    //let mut file = File::open("inputs/day15.test").unwrap();
    let mut file = File::open("inputs/day15.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

#[derive(Clone, Copy)]
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

#[derive(Clone)]
struct Grid(Vec<Vec<char>>);
impl Grid {
    #[allow(dead_code)]
    fn print(&self) {
        for line in &self.0 {
            println!("{}", line.iter().collect::<String>());
        }
    }

    fn make_move(&mut self, (i, j): (i32, i32), dir: Dir) -> Option<(i32, i32)> {
        let (di, dj) = dir.delta_index();
        let (ni, nj) = (i + di, j + dj);

        match (self.0[ni as usize][nj as usize], dir) {
            ('#', _) => return None,
            ('O', _) => {
                self.make_move((ni, nj), dir)?;
            }
            ('[', Dir::Up | Dir::Down) => {
                if self.make_move((ni, nj), dir).is_none()
                    || self.make_move((ni, nj + 1), dir).is_none()
                {
                    return None;
                }
            }
            (']', Dir::Up | Dir::Down) => {
                if self.make_move((ni, nj), dir).is_none()
                    || self.make_move((ni, nj - 1), dir).is_none()
                {
                    return None;
                }
            }
            ('[' | ']', _) => {
                self.make_move((ni, nj), dir)?;
            }

            _ => (),
        }

        self.swap(i as usize, j as usize, ni as usize, nj as usize);
        Some((ni, nj))
    }

    fn swap(&mut self, i: usize, j: usize, ni: usize, nj: usize) {
        let temp = self.0[i][j];
        self.0[i][j] = self.0[ni][nj];
        self.0[ni][nj] = temp;
    }
}

fn part_one(data: String) {
    let (grid, commands) = data.split_once("\n\n").unwrap();
    let mut grid = Grid(
        grid.lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
    let mut pos = grid
        .0
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .position(|&x| x == '@')
                .map(|j| (i as i32, j as i32))
        })
        .unwrap();

    for dir in commands.chars() {
        pos = match dir {
            '<' => grid.make_move(pos, Dir::Left).unwrap_or(pos),
            '>' => grid.make_move(pos, Dir::Right).unwrap_or(pos),
            '^' => grid.make_move(pos, Dir::Up).unwrap_or(pos),
            'v' => grid.make_move(pos, Dir::Down).unwrap_or(pos),
            _ => pos,
        };
    }

    let result: usize = grid
        .0
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter().enumerate().filter_map(
                move |(j, &value)| {
                    if value == 'O' {
                        Some((i, j))
                    } else {
                        None
                    }
                },
            )
        })
        .map(|(i, j)| 100 * i + j)
        .sum();

    println!("Result part 1: {result}");
}

fn part_two(data: String) {
    let (grid, commands) = data.split_once("\n\n").unwrap();
    let mut grid = Grid(
        grid.lines()
            .map(|line| {
                line.chars()
                    .flat_map(|c| match c {
                        '@' => "@.".chars(),
                        '#' => "##".chars(),
                        'O' => "[]".chars(),
                        _ => "..".chars(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );

    let mut pos = grid
        .0
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .position(|&x| x == '@')
                .map(|j| (i as i32, j as i32))
        })
        .unwrap();

    for dir in commands.chars() {
        pos = match dir {
            '<' => grid.make_move(pos, Dir::Left).unwrap_or(pos),
            '>' => grid.make_move(pos, Dir::Right).unwrap_or(pos),
            '^' => {
                let shallow_grid = grid.clone();

                if let Some(new_pos) = grid.make_move(pos, Dir::Up) {
                    new_pos
                } else {
                    grid = shallow_grid;
                    pos
                }
            }
            'v' => {
                let shallow_grid = grid.clone();

                if let Some(new_pos) = grid.make_move(pos, Dir::Down) {
                    new_pos
                } else {
                    grid = shallow_grid;
                    pos
                }
            }
            _ => pos,
        };
    }

    let result: usize = grid
        .0
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter().enumerate().filter_map(
                move |(j, &value)| {
                    if value == '[' {
                        Some((i, j))
                    } else {
                        None
                    }
                },
            )
        })
        .map(|(i, j)| 100 * i + j)
        .sum();

    println!("Result part 2: {result}");
}
