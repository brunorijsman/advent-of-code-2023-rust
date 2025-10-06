use std::fs::read_to_string;

fn extract_red_green_blue(draw: &str) -> (u32, u32, u32) {
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;
    for count_color in draw.split(',') {
        let (count, color) = count_color.trim().split_once(' ').unwrap();
        let count: u32 = count.parse().unwrap();
        match color {
            "red" => red += count,
            "green" => green += count,
            "blue" => blue += count,
            _ => panic!("Unknown color: {color}"),
        }
    }
    (red, green, blue)
}

fn max_red_green_blue(draws: &str) -> (u32, u32, u32) {
    let mut max_red: u32 = 0;
    let mut max_green: u32 = 0;
    let mut max_blue: u32 = 0;
    for draw in draws.split(';') {
        let (red, green, blue) = extract_red_green_blue(draw);
        max_red = max_red.max(red);
        max_green = max_green.max(green);
        max_blue = max_blue.max(blue);  
    }
    (max_red, max_green, max_blue)
}

fn main() {
    let mut sum = 0;
    for line in read_to_string("puzzle_input").unwrap().lines() {
        let (_, draws) = line.split_once(':').unwrap();
        let (max_red, max_green, max_blue) = max_red_green_blue(draws);
        let power = max_red * max_green * max_blue;
        sum += power;
    }
    println!("Sum of powers: {sum}");
}