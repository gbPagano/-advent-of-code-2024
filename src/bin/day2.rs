use std::fs::File;
use std::io::Read;

// difficulty: 2

fn main() {
    //let mut file = File::open("inputs/day2.test").unwrap();
    let mut file = File::open("inputs/day2.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

fn part_one(data: String) {
    let reports: Vec<_> = data
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let safe_reports = reports
        .iter()
        .filter(|report| is_valid_report(report).is_ok())
        .count();

    println!("Result part 1: {safe_reports}");
}

fn is_valid_report(report: &Vec<i32>) -> Result<(), i32> {
    let mut direction = 0;
    for (idx, window) in report.windows(2).enumerate() {
        if let &[a, b] = window {
            let diff = a - b;
            if diff.abs() == 0 || diff.abs() > 3 {
                return Err(idx as i32);
            }
            let new_direction = diff.signum();
            if direction == 0 {
                direction = new_direction;
            } else if direction != new_direction {
                return Err(idx as i32);
            }
        }
    }
    Ok(())
}

fn part_two(data: String) {
    let reports: Vec<_> = data
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut safe_reports = 0;
    for report in &reports {
        if let Err(idx) = is_valid_report(report) {
            for delta in -1..=1 {
                if (idx + delta) < 0 {
                    continue;
                }
                let mut new_report = report.clone();
                new_report.remove((idx + delta) as usize);
                if is_valid_report(&new_report).is_ok() {
                    safe_reports += 1;
                    break;
                }
            }
        } else {
            safe_reports += 1;
        }
    }
    println!("Result part 2: {safe_reports}");
}
