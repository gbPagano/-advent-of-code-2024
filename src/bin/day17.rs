use regex::Regex;
use std::fs::File;
use std::io::Read;

// difficulty: 5

fn main() {
    //let mut file = File::open("inputs/day17.test").unwrap();
    let mut file = File::open("inputs/day17.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

fn combo(regs: &[u64], x: u64) -> u64 {
    match x {
        0..=3 => x,
        4 => regs[0],
        5 => regs[1],
        6 => regs[2],
        _ => panic!("Invalid Program"),
    }
}

fn execute_program(regs: &mut [u64], program: &[u64]) -> Vec<u64> {
    let mut output = Vec::new();

    let mut pointer = 0;
    while pointer < program.len() - 1 {
        let ins = program[pointer];
        let operand = program[pointer + 1];

        match ins {
            0 => regs[0] >>= combo(regs, operand),   // adv
            1 => regs[1] ^= operand,                 // bxl
            2 => regs[1] = combo(regs, operand) % 8, // bst
            3 => {
                if regs[0] != 0 {
                    pointer = operand as usize; // jnz
                    continue;
                }
            }
            4 => regs[1] ^= regs[2],                        // bxc
            5 => output.push(combo(regs, operand) % 8),     // out
            6 => regs[1] = regs[0] >> combo(regs, operand), // bdv
            7 => regs[2] = regs[0] >> combo(regs, operand), // cdv
            _ => unreachable!(),
        }
        pointer += 2;
    }

    output
}

fn part_one(data: String) {
    let re = Regex::new(r"\d+").unwrap();
    let nums: Vec<_> = re
        .find_iter(&data)
        .filter_map(|item| item.as_str().parse::<u64>().ok())
        .collect();

    let mut regs = nums[..3].to_vec();
    let program = nums[3..].to_vec();

    let output = execute_program(&mut regs, &program);

    let result = output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");
    println!("Result part 1: {result}");
}

fn part_two(data: String) {
    let re = Regex::new(r"\d+").unwrap();
    let nums: Vec<_> = re
        .find_iter(&data)
        .filter_map(|item| item.as_str().parse::<u64>().ok())
        .collect();

    let program = nums[3..].to_vec();

    // assumptions
    // ends with a loop
    assert_eq!([3, 0], program[program.len() - 2..]);
    // only update once at loop
    assert_eq!(program.iter().step_by(2).filter(|&&x| x == 5).count(), 1);
    // only modify regs A once
    assert_eq!(program.iter().step_by(2).filter(|&&x| x == 0).count(), 1);
    let p = program.iter().step_by(2).position(|&x| x == 0).unwrap();
    assert_eq!(program[p * 2 + 1], 3); // and it is divided by 8

    let mut candidates = vec![0];
    for i in 0..program.len() {
        let mut next_candidates = Vec::new();
        for candidate in candidates {
            for j in 0..8 {
                let next = candidate << 3 | j;
                if execute_program(&mut [next, 0, 0], &program) == program[program.len() - 1 - i..]
                {
                    next_candidates.push(next);
                }
            }
        }
        candidates = next_candidates;
    }

    let result = candidates[0];
    println!("Result part 2: {result}");
}
