use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

// difficulty: 5

fn main() {
    //let mut file = File::open("inputs/day24.test").unwrap();
    let mut file = File::open("inputs/day24.input").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    part_one(data.clone());
    part_two(data);
}

#[derive(Debug, PartialEq)]
enum Operation {
    And,
    Or,
    Xor,
}
impl Operation {
    fn from_str(input: &str) -> Self {
        match input {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Connection<'a> {
    x: &'a str,
    op: Operation,
    y: &'a str,
    z: &'a str,
}
impl Connection<'_> {
    fn evaluate(&self, map: &HashMap<&str, usize>) -> Option<usize> {
        let x = map.get(self.x)?;
        let y = map.get(self.y)?;
        match self.op {
            Operation::And => Some(x & y),
            Operation::Or => Some(x | y),
            Operation::Xor => Some(x ^ y),
        }
    }
}

fn part_one(data: String) {
    let wires_re = Regex::new(r"(.+): (\d)").unwrap();
    let conn_re = Regex::new(r"(.+) (.+) (.+) .+ (.+)").unwrap();
    let (start_wires, raw_connections) = data.split_once("\n\n").unwrap();

    let mut wires: HashMap<&str, usize> = HashMap::new();
    for capture in wires_re.captures_iter(start_wires) {
        let key = capture.get(1).unwrap().as_str();
        let val = capture.get(2).unwrap().as_str();
        wires.insert(key, val.parse().unwrap());
    }

    let mut connections = VecDeque::new();
    for capture in conn_re.captures_iter(raw_connections) {
        let x = capture.get(1).unwrap().as_str();
        let op = Operation::from_str(capture.get(2).unwrap().as_str());
        let y = capture.get(3).unwrap().as_str();
        let z = capture.get(4).unwrap().as_str();

        connections.push_back(Connection { x, op, y, z });
    }

    while let Some(conn) = connections.pop_front() {
        if let Some(val) = conn.evaluate(&wires) {
            wires.insert(conn.z, val);
        } else {
            connections.push_back(conn);
        }
    }

    let mut z_wires: Vec<_> = wires.iter().filter(|(k, _)| k.starts_with("z")).collect();
    z_wires.sort();

    let result = z_wires
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, x)| acc | x.1 << idx);

    println!("Result part 1: {result}");
}

fn is_xy(conn: &Connection) -> bool {
    conn.x.starts_with("x") && conn.y.starts_with("y")
        || conn.x.starts_with("y") && conn.y.starts_with("x")
}

fn conn_is_valid(conn: &Connection, connections: &VecDeque<Connection>, max_z: &str) -> bool {
    if (conn.z.starts_with("z") && conn.op != Operation::Xor && conn.z != max_z)
        || conn.z == max_z && conn.op != Operation::Or
    {
        return false;
    } else if is_xy(conn) && conn.z != "z00" && !(conn.x.ends_with("00") && conn.y.ends_with("00"))
    {
        if (conn.op == Operation::Xor
            && connections
                .iter()
                .filter(|c| {
                    [c.x, c.y].contains(&conn.z) && [Operation::Xor, Operation::And].contains(&c.op)
                })
                .count()
                != 2)
            || conn.op == Operation::Or
            || (conn.op == Operation::And
                && connections
                    .iter()
                    .filter(|c| [c.x, c.y].contains(&conn.z) && c.op == Operation::Or)
                    .count()
                    == 0)
        {
            return false;
        }
    } else if !conn.z.starts_with("z") && !is_xy(conn) && conn.op == Operation::Xor {
        return false;
    }
    true
}

fn part_two(data: String) {
    let wires_re = Regex::new(r"(.+): (\d)").unwrap();
    let conn_re = Regex::new(r"(.+) (.+) (.+) .+ (.+)").unwrap();
    let (start_wires, raw_connections) = data.split_once("\n\n").unwrap();

    let mut wires: HashMap<&str, usize> = HashMap::new();
    for capture in wires_re.captures_iter(start_wires) {
        let key = capture.get(1).unwrap().as_str();
        let val = capture.get(2).unwrap().as_str();
        wires.insert(key, val.parse().unwrap());
    }

    let mut connections = VecDeque::new();
    for capture in conn_re.captures_iter(raw_connections) {
        let x = capture.get(1).unwrap().as_str();
        let op = Operation::from_str(capture.get(2).unwrap().as_str());
        let y = capture.get(3).unwrap().as_str();
        let z = capture.get(4).unwrap().as_str();

        connections.push_back(Connection { x, op, y, z });
    }

    let max_z = connections
        .iter()
        .filter_map(|item| {
            if item.z.starts_with("z") {
                Some(item.z)
            } else {
                None
            }
        })
        .max()
        .unwrap();

    let mut wrongs = Vec::new();
    for conn in &connections {
        if !conn_is_valid(conn, &connections, max_z) {
            wrongs.push(conn.z);
        }
    }
    wrongs.sort();

    let result = wrongs.join(",");
    println!("Result part 2: {result}");
}
