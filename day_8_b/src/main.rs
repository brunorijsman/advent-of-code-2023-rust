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
    let steps = go_all_xxa_to_all_xxz(&network, &directions);
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

fn go_all_xxa_to_all_xxz(network: &HashMap<String, Node>, directions: &str) -> u64 {
    let mut each_xxa_steps = Vec::new();
    for start_name in all_xxa(network) {
        let steps = go_xxa_to_xxz(network, &start_name, directions);
        each_xxa_steps.push(steps);
    }
    each_xxa_steps.iter().fold(1, |acc, &x| least_common_multiple(acc, x))
}


fn go_xxa_to_xxz(network: &HashMap<String, Node>, start_name: &String, directions: &str) -> u64 {
    //
    // This solution relies on the fact that the problem input has been carefully crafter so that
    // every path from an 'XXA' node goes through a period cycle visiting the same XXZ node over
    // and over again, using the same number of initial steps and the same number of steps in the cycle.
    //
    // Using this fact, we just find the cycle length for each XXA and then compute the least common
    // multiple of all the cycle lengths to get the answer.
    //
    let mut steps: u64 = 0;
    let mut current_name: String = start_name.clone();
    let mut direction_index = 0;
    loop {
        if current_name.ends_with('Z') {
            return steps;
        }
        let direction_char = directions.chars().nth(direction_index).unwrap();
        let node = network.get(&current_name).unwrap();
        current_name = match direction_char {
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

fn all_xxa(network: &HashMap<String, Node>) -> Vec<String> {
    network
        .keys()
        .filter(|name| name.ends_with('A'))
        .cloned()
        .collect()
}

fn least_common_multiple(a: u64, b: u64) -> u64 {
    let mut multiple = a;
    while multiple % b != 0 {
        multiple += a;
    }
    multiple
}   
