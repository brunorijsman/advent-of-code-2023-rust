use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("puzzle_input").unwrap();
    let mut reader = BufReader::new(file);
    let times = read_numbers(&mut reader);
    let distances = read_numbers(&mut reader);
    assert!(times.len() == distances.len());
    bruce_force_solve(&times, &distances);
    math_solve(&times, &distances);
}

fn read_numbers(reader: &mut BufReader<File>) -> Vec<u64> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let (_, numbers) = line.split_once(':').unwrap();
    let numbers = numbers
        .split_whitespace()
        .map(|n| n.trim().parse::<u64>().unwrap())
        .collect();
    numbers
}

fn bruce_force_solve(times: &[u64], distances: &[u64]) {
    let mut product = 1u64;
    for (time, distance) in times.iter().zip(distances.iter()) {
        let ways = brute_force_nr_ways_to_win(*time, *distance);
        product *= ways;
    }
    println!("Brute force solution product ways: {product}");
}

fn brute_force_nr_ways_to_win(max_time: u64, min_distance: u64) -> u64 {
    let mut count = 0;
    for charge_time in 1..=max_time {
        if charge_time_is_feasible(max_time, charge_time, min_distance) {
            count += 1;
        }
    }
    count
}

fn charge_time_is_feasible(max_time: u64, charge_time: u64, min_distance: u64) -> bool {
    let distance = distance_for_charge_time(max_time, charge_time);
    distance > min_distance
}

fn distance_for_charge_time(max_time: u64, charge_time: u64) -> u64 {
    let speed = charge_time;
    let remaining_time = max_time - charge_time;
    let distance = speed * remaining_time;
    distance
}

fn math_solve(times: &[u64], distances: &[u64]) {
    let mut product = 1u64;
    for (time, distance) in times.iter().zip(distances.iter()) {
        let ways = math_nr_ways_to_win(*time, *distance);
        product *= ways;
    }
    println!("Math solution product ways: {product}");
}

fn math_nr_ways_to_win(max_time: u64, min_distance: u64) -> u64 {
    //
    // Actual distance traveled is a quadratic function of charge time:
    // 
    // distance = speed * time
    //          = charge_time * (max_time - charge_time)
    //          = charge_time * max_time - charge_time^2
    //          = -charge_time^2 + max_time * charge_time
    //
    // For the record to be beaten:
    //
    // distance > min_distance =>
    // -charge_time^2 + max_time * charge_time > min_distance =>
    // charge_time^2 - max_time * charge_time + min_distance < 0
    //
    // The quadratic function has two roots:
    //
    // charge_time = (max_time +- sqrt(max_time^2 - 4 * min_distance)) / 2
    //
    let discriminant = max_time as f64 * max_time as f64 - 4.0 * min_distance as f64;
    assert!(discriminant >= 0.0);
    let sqrt_discriminant = discriminant.sqrt();
    let root1 = (max_time as f64 - sqrt_discriminant) / 2.0;
    let root2 = (max_time as f64 + sqrt_discriminant) / 2.0;
    let mut min_feasible_time = root1.ceil() as u64;
    let mut max_feasible_time = root2.floor() as u64;
    // Explicitly check that we exceed the minimum distance at the boundaries
    if !charge_time_is_feasible(max_time, min_feasible_time, min_distance) {
        min_feasible_time += 1;
    }
    if !charge_time_is_feasible(max_time, max_feasible_time, min_distance) {
        max_feasible_time -= 1;
    }
    if min_feasible_time > max_feasible_time {
        return 0;
    }
    max_feasible_time - min_feasible_time + 1
}

