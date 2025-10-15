use std::fs::read_to_string;

struct Lens {
    label: String,
    focal_length: u64,
}

type Box = Vec<Lens>;

type Boxes = Vec<Box>;

const NR_BOXES: usize = 256;

fn main() {
    let init_seq = read_init_seq();
    let mut boxes = Boxes::new();
    for _ in 0..NR_BOXES {
        boxes.push(Box::new());
    }
    run_init_seq(&init_seq, &mut boxes);
    let total = total_focusing_power(&boxes);
    println!("Total focusing power: {}", total);
}

fn read_init_seq() -> Vec<String> {
    let contents = read_to_string("puzzle_input").unwrap();
    let mut lines = contents.lines();
    lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.to_string())
        .collect()
}

fn run_init_seq(init_seq: &Vec<String>, boxes: &mut Boxes) {
    for step in init_seq {
        run_step(&step, boxes);
    }
}

fn run_step(step: &String, boxes: &mut Boxes) {
    if step.ends_with("-") {
        remove_lens(step, boxes);
    } else {
        add_lens(step, boxes);
    }
}

fn remove_lens(step: &String, boxes: &mut Boxes) {
    let mut label = step.clone();
    label.pop().unwrap();
    let hash = hash(&label.to_string()) as usize;
    if let Some(pos) = boxes[hash].iter().position(|lens| lens.label == label) {
        boxes[hash].remove(pos);
    }
}

fn add_lens(step: &String, boxes: &mut Boxes) {
    let (label, focal_length) = step.split_once('=').unwrap();
    let label = label.to_string();
    let focal_length: u64 = focal_length.parse().unwrap();
    let lens = Lens {
        label: label.clone(),
        focal_length,
    };
    let hash = hash(&label) as usize;
    for existing_lens in &mut boxes[hash] {
        if existing_lens.label == label {
            existing_lens.focal_length = focal_length;
            return;
        }
    }
    boxes[hash].push(lens);
}

fn hash(s: &String) -> u64 {
    let bytes = s.as_bytes();
    let mut hash: u64 = 0;
    for byte in bytes {
        hash += *byte as u64;
        hash *= 17;
        hash %= 256;
    }
    hash
}

fn total_focusing_power(boxes: &Boxes) -> usize {
    let mut total = 0;
    for (box_nr, the_box) in boxes.iter().enumerate() {
        for (lens_nr, lens) in the_box.iter().enumerate() {
            let focusing_power = (box_nr + 1) * (lens_nr + 1) * (lens.focal_length as usize);
            total += focusing_power;
        }
    }
    total
}
