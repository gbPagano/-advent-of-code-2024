use std::fmt;
use std::fs::File;
use std::io::Read;

// difficulty: 4

fn main() {
    //let mut file = File::open("inputs/day9.test").unwrap();
    let mut file = File::open("inputs/day9.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum State {
    Empty(u32),
    File(u32),
}
impl State {
    fn next(self) -> Self {
        match self {
            State::Empty(idx) => State::File(idx + 1),
            State::File(idx) => State::Empty(idx),
        }
    }
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Empty(_) => write!(f, "."),
            State::File(val) => write!(f, "{}", val),
        }
    }
}

#[derive(Debug)]
enum Block {
    Empty(u32),       // size
    File((u32, u32)), // (val, size)
}
impl Block {
    fn from_state(state: State, size: usize) -> Self {
        match state {
            State::Empty(_) => Block::Empty(size as u32),
            State::File(val) => Block::File((val, size as u32)),
        }
    }

    fn get_size(&self) -> u32 {
        match self {
            Block::Empty(size) => *size,
            Block::File((_, size)) => *size,
        }
    }
}

fn part_one(data: String) {
    let (_, mut disk) = data
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .fold((State::File(0), Vec::new()), |(state, mut disk), num| {
            disk.extend(std::iter::repeat(state).take(num));
            (state.next(), disk)
        });

    let (mut i, mut j) = (0, disk.len() - 1);
    while i < j {
        if let State::File(_) = disk[i] {
            i += 1;
            continue;
        }
        if let State::Empty(_) = disk[j] {
            j -= 1;
            continue;
        }
        disk.swap(i, j);
    }

    let mut result = 0;
    for (idx, item) in disk.iter().enumerate() {
        if let State::File(val) = item {
            result += idx * *val as usize;
        } else {
            break;
        }
    }
    println!("Result part 1: {result}");
}

fn part_two(data: String) {
    let (_, disk) = data
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .fold((State::File(0), Vec::new()), |(state, mut disk), num| {
            disk.extend(std::iter::repeat(state).take(num));
            (state.next(), disk)
        });

    let mut block_disk = Vec::new();
    let mut last_item = (disk[0], 0); // (state, idx)
    for (idx, val) in disk.iter().enumerate() {
        if *val != last_item.0 {
            block_disk.push(Block::from_state(last_item.0, idx - last_item.1));
            last_item = (*val, idx);
        }
        if idx == disk.len() - 1 {
            block_disk.push(Block::from_state(last_item.0, idx - last_item.1 + 1));
        }
    }

    for i in 0..block_disk.len() {
        let i_size = match block_disk[i] {
            Block::Empty(val) => val,
            Block::File(_) => continue,
        };
        for j in (i + 1..block_disk.len()).rev() {
            match block_disk[j] {
                Block::Empty(_) => continue,
                Block::File((_, size)) if size > i_size => continue,
                Block::File((_, size)) if size == i_size => {
                    block_disk.swap(i, j);
                    break;
                }
                Block::File((_, size)) if size < i_size => {
                    let file = block_disk.remove(j);
                    let empty = block_disk.remove(i);

                    block_disk.insert(i, Block::Empty(empty.get_size() - file.get_size()));
                    block_disk.insert(j, Block::Empty(file.get_size()));

                    block_disk.insert(i, file);
                    break;
                }
                _ => unreachable!(),
            }
        }
    }
    let mut result = 0;
    let mut idx = 0;
    for block in block_disk {
        match block {
            Block::Empty(size) => idx += size,
            Block::File((val, size)) => {
                result += (0..size).map(|i| (val * (i + idx)) as u64).sum::<u64>();
                idx += size;
            }
        }
    }
    println!("Result part 2: {result}");
}
