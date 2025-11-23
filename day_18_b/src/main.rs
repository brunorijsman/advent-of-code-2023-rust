// See analysis.pptx for explanation

use std::fs::read_to_string;

fn main() {
    let mut y: isize = 0;
    let mut line_area: isize = 0;
    let mut exterior: isize = 0;
    let lines = read_to_string("puzzle_input").unwrap();
    for line in lines.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert!(parts.len() == 3);
        let instructions = parts[2].to_string();
        let direction = instructions.chars().nth(7).unwrap();
        let length = isize::from_str_radix(&instructions[2..7], 16).unwrap();
        exterior += length;
        match direction {
            '0' => line_area += length * y,
            '1' => y -= length,
            '2' => line_area -= length * y,
            '3' => y += length,
            _ => panic!("Unknown direction"),
        };
    }
    let line_area = line_area.abs();
    let interior = line_area - exterior / 2 + 1;
    let trench_area = interior + exterior;
    println!("Area: {trench_area}");
}
