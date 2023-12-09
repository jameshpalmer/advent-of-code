use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Iterator for Direction {
    type Item = Direction;

    fn next(&mut self) -> Option<Direction> {
        match self {
            Direction::Left => Some(Direction::Right),
            Direction::Right => Some(Direction::Left),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let directions: Vec<Direction> = input
        .split("\n\n")
        .nth(0)
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        })
        .collect();

    let node_strs: Vec<&str> = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .split("\n")
        .collect::<Vec<&str>>();

    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    for node_str in node_strs.clone() {
        let values_str = node_str
            .chars()
            .filter(|&c| c.is_alphabetic() || c.is_whitespace())
            .collect::<String>();

        let values = values_str.split_whitespace().collect::<Vec<&str>>();
        let name = values[0];
        let left = values[1];
        let right = values[2];

        nodes.insert(name.to_string(), (left.to_string(), right.to_string()));
    }

    let mut current_node = nodes.get("AAA").unwrap();
    let mut direction_count = 0;

    'outer: loop {
        for direction in &directions {
            direction_count += 1;

            match direction {
                Direction::Left => {
                    if current_node.0 == "ZZZ" {
                        break 'outer;
                    }
                    current_node = nodes.get(&current_node.0).unwrap();
                }
                Direction::Right => {
                    if current_node.1 == "ZZZ" {
                        break 'outer;
                    }
                    current_node = nodes.get(&current_node.1).unwrap();
                }
            }
        }
    }

    println!("{:?}", direction_count);
}
