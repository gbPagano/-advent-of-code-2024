use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

// difficulty: 4

fn main() {
    //let mut file = File::open("inputs/day14.test").unwrap();
    let mut file = File::open("inputs/day14.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

const GRID_SIZE: (i32, i32) = (103, 101);

#[derive(Debug)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}
impl Robot {
    fn step(&mut self, steps: i32, bathroom_size: (i32, i32)) {
        let (nx, ny) = (self.px + self.vx * steps, self.py + self.vy * steps);
        self.px = nx - (bathroom_size.0 * (nx as f32 / bathroom_size.0 as f32).floor() as i32);
        self.py = ny - (bathroom_size.1 * (ny as f32 / bathroom_size.1 as f32).floor() as i32);
    }
}

fn part_one(data: String) {
    let re = Regex::new(r"-?\d+").unwrap();
    let mut robots: Vec<_> = data
        .lines()
        .map(|line| {
            let nums: Vec<_> = re
                .find_iter(line)
                .filter_map(|item| item.as_str().parse::<i32>().ok())
                .collect();
            Robot {
                px: nums[1],
                py: nums[0],
                vx: nums[3],
                vy: nums[2],
            }
        })
        .collect();

    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for robot in robots.iter_mut() {
        robot.step(100, GRID_SIZE);

        q1 += (robot.px < GRID_SIZE.0 / 2 && robot.py < GRID_SIZE.1 / 2) as i32;
        q2 += (robot.px < GRID_SIZE.0 / 2 && robot.py > GRID_SIZE.1 / 2) as i32;
        q3 += (robot.px > GRID_SIZE.0 / 2 && robot.py > GRID_SIZE.1 / 2) as i32;
        q4 += (robot.px > GRID_SIZE.0 / 2 && robot.py < GRID_SIZE.1 / 2) as i32;
    }

    let result = q1 * q2 * q3 * q4;
    println!("Result part 1: {result}");
}

fn part_two(data: String) {
    let re = Regex::new(r"-?\d+").unwrap();
    let mut robots: Vec<_> = data
        .lines()
        .map(|line| {
            let nums: Vec<_> = re
                .find_iter(line)
                .filter_map(|item| item.as_str().parse::<i32>().ok())
                .collect();
            Robot {
                px: nums[1],
                py: nums[0],
                vx: nums[3],
                vy: nums[2],
            }
        })
        .collect();

    let mut min_steps = 0;
    loop {
        let mut unique_pos = HashSet::new();
        for robot in robots.iter_mut() {
            robot.step(1, GRID_SIZE);
            unique_pos.insert((robot.px, robot.py));
        }
        min_steps += 1;
        if unique_pos.len() == robots.len() {
            break;
        }
    }

    let mut final_grid = [[' '; GRID_SIZE.1 as usize]; GRID_SIZE.0 as usize];
    for r in robots {
        final_grid[r.px as usize][r.py as usize] = '#';
    }
    for line in final_grid {
        println!("{}", line.iter().collect::<String>());
    }

    println!("Result part 2: {min_steps}");
}
