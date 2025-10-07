use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct MapItem {
    dst_range_start: usize,
    src_range_start: usize,
    range_len: usize,
}

type Map = Vec<MapItem>;

type MapSeq = Vec<Map>;

fn main() {
    let file = File::open("puzzle_input").unwrap();
    let mut reader = BufReader::new(file);
    let seeds = read_seeds(&mut reader);
    let map_seq = read_map_seq(&mut reader, 7);
    let locations = seeds
        .iter()
        .map(|s| apply_map_seq(&map_seq, *s))
        .collect::<Vec<usize>>();
    let closest = locations.iter().min().unwrap();
    println!("Closest location: {closest}");
}

fn read_seeds(reader: &mut BufReader<File>) -> Vec<usize> {
    let line = next_line(reader).unwrap();
    let seeds = &line[7..]
        .split_whitespace()
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    next_line(reader).unwrap(); // skip blank line
    seeds.to_vec()
}

fn read_map(reader: &mut BufReader<File>) -> Map {
    let mut map: Map = Map::new();
    next_line(reader).unwrap(); // skip foo-to-bar map: line
    while let Some(line) = next_line(reader) {
        if line.is_empty() {
            break;
        }
        let numbers = line
            .split_whitespace()
            .map(|n| n.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        assert!(numbers.len() == 3);
        let map_item = MapItem {
            dst_range_start: numbers[0],
            src_range_start: numbers[1],
            range_len: numbers[2],
        };
        map.push(map_item);
    }
    map
}

fn next_line(reader: &mut BufReader<File>) -> Option<String> {
    let mut line = String::new();
    let bytes_read = reader.read_line(&mut line).unwrap();
    if bytes_read == 0 {
        return None;
    }
    line = line.trim().to_string();
    Some(line)
}
	
fn read_map_seq(reader: &mut BufReader<File>, nr_maps: usize) -> MapSeq {
    let mut map_seq = MapSeq::new();
    for _ in 0..nr_maps {
        let map = read_map(reader);
        map_seq.push(map);
    }
    map_seq
}

fn apply_map_seq(map_seq: &MapSeq, nr: usize) -> usize {
    let mut current = nr;
    for map in map_seq {
        current = apply_map(map, current);
    }
    current
}

fn apply_map(map: &Map, nr: usize) -> usize {
    for item in map {
        if nr >= item.src_range_start && nr < item.src_range_start + item.range_len {
            let offset = nr - item.src_range_start;
            return item.dst_range_start + offset;
        }
    }
    nr
}
