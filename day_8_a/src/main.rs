use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

fn main() {
    let file = File::open("puzzle_input").unwrap();
    let mut reader = BufReader::new(file);
    let directions = read_directions(&mut reader);
    let network = read_network(&mut reader);
    let steps = go_aaa_to_zzz(&network, &directions);
    println!("Steps: {steps}");
}

fn read_directions(reader: &mut BufReader<File>) -> String {
    let mut line: String = String::new();
    reader.read_line(&mut line).unwrap();
    let directions = line.trim().to_string();
    // Skip the empty line
    reader.read_line(&mut line).unwrap();
    directions
}

fn read_network(reader: &mut BufReader<File>) -> HashMap<String, Node> {
    let mut network = HashMap::new();
    while let Some(node) = read_node(reader) {
        network.insert(node.name.clone(), node);
    }
    network
}

fn read_node(reader: &mut BufReader<File>) -> Option<Node> {
    let mut line: String = String::new();
    if reader.read_line(&mut line).unwrap() == 0 {
        return None;
    }
    let name = line[0..3].to_string();
    let left = line[7..10].to_string();
    let right = line[12..15].to_string();
    Some(Node { name, left, right })
}

fn go_aaa_to_zzz(network: &HashMap<String, Node>, directions: &str) -> u64 {
    let mut steps: u64 = 0;
    let mut current = "AAA".to_string();
    let mut direction_index = 0;
    loop {
        if current == "ZZZ" {
            return steps;
        }
        let node = network.get(&current).unwrap();
        let direction_char = directions.chars().nth(direction_index).unwrap();
        current = match direction_char {
            'L' => node.left.clone(),
            'R' => node.right.clone(),
            _ => panic!("Invalid direction: {direction_char}"),
        };
        direction_index += 1;
        if direction_index >= directions.len() {
            direction_index = 0;
        }
        steps += 1;
    }
}
