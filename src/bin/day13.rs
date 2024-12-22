use regex::Regex;
use std::fs::File;
use std::io::Read;

// difficulty: 3

fn main() {
    //let mut file = File::open("inputs/day13.test").unwrap();
    let mut file = File::open("inputs/day13.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

#[derive(Debug)]
struct Machine {
    ax: isize,
    ay: isize,
    bx: isize,
    by: isize,
    px: isize,
    py: isize,
}

impl Machine {
    fn solve(&self) -> Option<isize> {
        let d = self.ax * self.by - self.ay * self.bx;
        let di = self.px * self.by - self.py * self.bx;
        let dj = self.py * self.ax - self.px * self.ay;

        if di % d == 0 && dj % d == 0 {
            Some(3 * di / d + dj / d)
        } else {
            None
        }
    }
}

fn part_one(data: String) {
    let re = Regex::new(r"\d+").unwrap();
    let result = data
        .split("\n\n")
        .map(|block| {
            let captures: Vec<_> = re
                .find_iter(&block)
                .filter_map(|item| item.as_str().parse::<isize>().ok())
                .collect();
            Machine {
                ax: captures[0],
                ay: captures[1],
                bx: captures[2],
                by: captures[3],
                px: captures[4],
                py: captures[5],
            }
        })
        .filter_map(|machine| {
            machine.solve() 
        })
        .sum::<isize>();

    println!("Result part 1: {result}");
}

fn part_two(data: String) {
    let re = Regex::new(r"\d+").unwrap();
    let result = data
        .split("\n\n")
        .map(|block| {
            let captures: Vec<_> = re
                .find_iter(&block)
                .filter_map(|item| item.as_str().parse::<isize>().ok())
                .collect();
            Machine {
                ax: captures[0],
                ay: captures[1],
                bx: captures[2],
                by: captures[3],
                px: captures[4] + 10e12 as isize,
                py: captures[5] + 10e12 as isize,
            }
        })
        .filter_map(|machine| {
            machine.solve() 
        })
        .sum::<isize>();
    
    println!("Result part 2: {result}");
}
