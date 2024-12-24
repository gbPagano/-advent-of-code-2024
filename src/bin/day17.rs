use regex::Regex;
use std::fs::File;
use std::io::Read;

// difficulty: ?

fn main() {
    let mut file = File::open("inputs/day17.test").unwrap();
    let mut file = File::open("inputs/day17.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    //part_two(data);
}

fn combo(regs: &[u32], x: u32) -> u32 {
    match x {
        0..=3 => x,
        4 => regs[0],
        5 => regs[1],
        6 => regs[2],
        _ => panic!("Invalid Program"),
    }
}

fn part_one(data: String) {
    let re = Regex::new(r"\d+").unwrap();
    let nums: Vec<_> = re
        .find_iter(&data)
        .filter_map(|item| item.as_str().parse::<u32>().ok())
        .collect();

    let mut regs = nums[..3].to_vec();
    let program = nums[3..].to_vec();
    let mut output = Vec::new();
    
    let mut pointer = 0;
    while pointer < program.len() - 1 {
        let ins = program[pointer];
        let operand = program[pointer + 1];

        match ins {
            0 => regs[0] >>= combo(&regs, operand),   // adv
            1 => regs[1] ^= operand,                  // bxl
            2 => regs[1] = combo(&regs, operand) % 8, // bst
            3 => {
                if regs[0] != 0 {
                    pointer = operand as usize; // jnz
                    continue;
                }
            }
            4 => regs[1] ^= regs[2],                         // bxc
            5 => output.push(combo(&regs, operand) % 8),     // out
            6 => regs[1] = regs[0] >> combo(&regs, operand), // bdv
            7 => regs[2] = regs[0] >> combo(&regs, operand), // cdv
            _ => unreachable!(),
        }
        pointer += 2;
    }

    let result = output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");
    println!("Result part 1: {result}");
}
