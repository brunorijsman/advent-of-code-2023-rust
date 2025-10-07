use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Range {
    start: i64,
    len: i64,
}

type RangeSeq = Vec<Range>;

#[derive(Debug, Clone)]
struct MapItem {
    dst_range_start: i64,
    src_range_start: i64,
    range_len: i64,
}

type Map = Vec<MapItem>;

type MapSeq = Vec<Map>;

#[derive(Debug, Clone)]
struct IntermediateMapResult {
    mapped_ranges: RangeSeq,
    unmapped_ranges: RangeSeq,
}

fn main() {
    let file = File::open("puzzle_input").unwrap();
    let mut reader = BufReader::new(file);
    let seed_ranges = read_seed_ranges(&mut reader);
    let nr_maps = 7;
    let map_seq = read_map_seq(&mut reader, nr_maps);
    let mut result_ranges = apply_map_seq_to_range_seq(&map_seq, &seed_ranges);
    assert!(result_ranges.len() > 0);
    result_ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let closest_location = result_ranges[0].start;
    println!("Closest location: {}", closest_location);
}

fn read_seed_ranges(reader: &mut BufReader<File>) -> RangeSeq {
    let mut seed_ranges = RangeSeq::new();
    let line = next_line(reader).unwrap();
    let numbers = &line[7..]
        .split_whitespace()
        .map(|n| n.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    assert!(numbers.len() % 2 == 0);
    let nr_pairs = numbers.len() / 2;
    for i in 0..nr_pairs {
        let start = numbers[2 * i];
        let len = numbers[2 * i + 1];
        let range = Range { start, len };
        seed_ranges.push(range);
    }
    next_line(reader).unwrap(); // skip blank line
    seed_ranges
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
            .map(|n| n.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
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
	
fn read_map_seq(reader: &mut BufReader<File>, nr_maps: i64) -> MapSeq {
    let mut map_seq = MapSeq::new();
    for _ in 0..nr_maps {
        let map = read_map(reader);
        map_seq.push(map);
    }
    map_seq
}

fn apply_map_item_to_range(map_item: &MapItem, range: &Range) -> IntermediateMapResult {
    let mut result = IntermediateMapResult{
        mapped_ranges: RangeSeq::new(),
        unmapped_ranges: RangeSeq::new(),
    };
    if let Some(before_range) = sub_range_before_map_item(range, map_item) {
        // Sub-range before the map item is unaffected
        result.unmapped_ranges.push(before_range);
    }
    if let Some(overlap_range) = sub_range_overlap_map_item(range, map_item) {
        // Sub-range overlapping the map item is mapped
        let offset = map_item.dst_range_start - map_item.src_range_start;
        let mapped_overlap_range = Range {
            start: overlap_range.start + offset,
            len: overlap_range.len,
        };
        result.mapped_ranges.push(mapped_overlap_range);
    }
    if let Some(after_range) = sub_range_after_map_item(range, map_item) {
        // Sub-range after the map item is unaffected
        result.unmapped_ranges.push(after_range);
    }
    result
}

fn sub_range_before_map_item(range: &Range, map_item: &MapItem) -> Option<Range> {
    if range.start >= map_item.src_range_start {
        return None;
    }
    let range_end = range.start + range.len - 1;
    let before_start = range.start;
    let before_end = std::cmp::min(range_end, map_item.src_range_start - 1);
    let before_len = before_end - before_start + 1;
    Some(Range {start: before_start, len: before_len})
}

fn sub_range_overlap_map_item(range: &Range, map_item: &MapItem) -> Option<Range> {
    let range_end = range.start + range.len - 1;
    if range_end < map_item.src_range_start {
        return None;
    }
    let map_item_end = map_item.src_range_start + map_item.range_len - 1;
    if range.start > map_item_end {
        return None;
    }
    let overlap_start = std::cmp::max(range.start, map_item.src_range_start);
    let overlap_end = std::cmp::min(range_end, map_item_end);
    let overlap_len = overlap_end - overlap_start + 1;
    Some(Range {start: overlap_start, len: overlap_len})
}

fn sub_range_after_map_item(range: &Range, map_item: &MapItem) -> Option<Range> {
    let range_end = range.start + range.len - 1;
    let map_item_end = map_item.src_range_start + map_item.range_len - 1;
    if range_end <= map_item_end {
        return None;
    }
    let after_start = std::cmp::max(range.start, map_item_end + 1);
    let after_end = range_end;
    let after_len = after_end - after_start + 1;
    Some(Range {start: after_start, len: after_len})
}

fn apply_map_to_range(map: &Map, range: &Range) -> RangeSeq {
    let mut final_result = IntermediateMapResult{
        mapped_ranges: RangeSeq::new(),
        unmapped_ranges: vec![range.clone()],
    };
    for map_item in map {
        let mut step_result = apply_map_item_to_ranges(map_item, &final_result.unmapped_ranges);
        final_result.mapped_ranges.append(&mut step_result.mapped_ranges);
        final_result.unmapped_ranges = step_result.unmapped_ranges;
    }
    // Any ranges still unmapped after applying all map items remain unchanged
    let ranges = vec![final_result.mapped_ranges, final_result.unmapped_ranges].concat();
    ranges
}

fn apply_map_item_to_ranges(map_item: &MapItem, ranges: &RangeSeq) -> IntermediateMapResult {
    let mut final_result = IntermediateMapResult{
        mapped_ranges: RangeSeq::new(),
        unmapped_ranges: RangeSeq::new(),
    };
    for range in ranges {
        let mut step_result = apply_map_item_to_range(map_item, range);
        final_result.mapped_ranges.append(&mut step_result.mapped_ranges);
        final_result.unmapped_ranges.append(&mut step_result.unmapped_ranges);
    }
    final_result
}

fn apply_map_to_range_seq(map: &Map, ranges: &RangeSeq) -> RangeSeq {
    let mut result_ranges = RangeSeq::new();
    for range in ranges {
        let mut more_ranges = apply_map_to_range(map, range);
        result_ranges.append(&mut more_ranges);
    }
    result_ranges
}

fn apply_map_seq_to_range_seq(map_seq: &MapSeq, ranges: &RangeSeq) -> RangeSeq {
    let mut new_ranges = ranges.clone();
    for map in map_seq {
        new_ranges = apply_map_to_range_seq(map, &new_ranges);
    }
    new_ranges
}

#[test]
fn test_simple_subset() {
    let map_item = MapItem {
        dst_range_start: 200,
        src_range_start: 100,
        range_len: 50,
    };
    let map = vec![map_item.clone()];
    let map_seq = vec![map.clone()];
    let range = Range {
        start: 110, 
        len: 30, 
    };
    let range_seq = vec![range.clone()];
    let mapped_range_seq = apply_map_seq_to_range_seq(&map_seq, &range_seq);
    assert_eq!(mapped_range_seq.len(), 1);
    assert_eq!(mapped_range_seq[0].start, 210);
    assert_eq!(mapped_range_seq[0].len, 30);
}

#[test]
fn test_simple_fully_before() {
    let map_item = MapItem {
        dst_range_start: 200,
        src_range_start: 100,
        range_len: 50,
    };
    let map = vec![map_item.clone()];
    let map_seq = vec![map.clone()];
    let range = Range {
        start: 50, 
        len: 30, 
    };
    let range_seq = vec![range.clone()];
    let mapped_range_seq = apply_map_seq_to_range_seq(&map_seq, &range_seq);
    assert_eq!(mapped_range_seq.len(), 1);
    assert_eq!(mapped_range_seq[0].start, 50);
    assert_eq!(mapped_range_seq[0].len, 30);
}

#[test]
fn test_simple_fully_after() {
    let map_item = MapItem {
        dst_range_start: 200,
        src_range_start: 100,
        range_len: 50,
    };
    let map = vec![map_item.clone()];
    let map_seq = vec![map.clone()];
    let range = Range {
        start: 260, 
        len: 30, 
    };
    let range_seq = vec![range.clone()];
    let mapped_range_seq = apply_map_seq_to_range_seq(&map_seq, &range_seq);
    assert_eq!(mapped_range_seq.len(), 1);
    assert_eq!(mapped_range_seq[0].start, 260);
    assert_eq!(mapped_range_seq[0].len, 30);
}

#[test]
fn test_simple_begin_overlap() {
    let map_item = MapItem {
        dst_range_start: 200,
        src_range_start: 100,
        range_len: 50,
    };
    let map = vec![map_item.clone()];
    let map_seq = vec![map.clone()];
    let range = Range {
        start: 90, 
        len: 20, 
    };
    let range_seq = vec![range.clone()];
    let mapped_range_seq = apply_map_seq_to_range_seq(&map_seq, &range_seq);
    assert_eq!(mapped_range_seq.len(), 2);
    assert_eq!(mapped_range_seq[0].start, 200);
    assert_eq!(mapped_range_seq[0].len, 10);
    assert_eq!(mapped_range_seq[1].start, 90);
    assert_eq!(mapped_range_seq[1].len, 10);
}

#[test]
fn test_simple_end_overlap() {
    let map_item = MapItem {
        dst_range_start: 200,
        src_range_start: 100,
        range_len: 50,
    };
    let map = vec![map_item.clone()];
    let map_seq = vec![map.clone()];
    let range = Range {
        start: 140, 
        len: 20, 
    };
    let range_seq = vec![range.clone()];
    let mapped_range_seq = apply_map_seq_to_range_seq(&map_seq, &range_seq);
    assert_eq!(mapped_range_seq.len(), 2);
    assert_eq!(mapped_range_seq[0].start, 240);
    assert_eq!(mapped_range_seq[0].len, 10);
    assert_eq!(mapped_range_seq[1].start, 150);
    assert_eq!(mapped_range_seq[1].len, 10);
}

#[test]
fn test_simple_superset() {
    let map_item = MapItem {
        dst_range_start: 200,
        src_range_start: 100,
        range_len: 50,
    };
    let map = vec![map_item.clone()];
    let map_seq = vec![map.clone()];
    let range = Range {
        start: 90, 
        len: 70, 
    };
    let range_seq = vec![range.clone()];
    let mapped_range_seq = apply_map_seq_to_range_seq(&map_seq, &range_seq);
    assert_eq!(mapped_range_seq.len(), 3);
    assert_eq!(mapped_range_seq[0].start, 200);
    assert_eq!(mapped_range_seq[0].len, 50);
    assert_eq!(mapped_range_seq[1].start, 90);
    assert_eq!(mapped_range_seq[1].len, 10);
    assert_eq!(mapped_range_seq[2].start, 150);
    assert_eq!(mapped_range_seq[2].len, 10);
}

#[test]
fn test_multi_item_superset() {
    let map_item_1 = MapItem {
        dst_range_start: 1000,
        src_range_start: 100,
        range_len: 50,
    };
    let map_item_2 = MapItem {
        dst_range_start: 2000,
        src_range_start: 85,
        range_len: 10,
    };
    let map = vec![map_item_1.clone(), map_item_2.clone()];
    let map_seq = vec![map.clone()];
    let range = Range {
        start: 90, 
        len: 70, 
    };
    let range_seq = vec![range.clone()];
    let mapped_range_seq = apply_map_seq_to_range_seq(&map_seq, &range_seq);
    assert_eq!(mapped_range_seq.len(), 4);
    assert_eq!(mapped_range_seq[0].start, 1000);
    assert_eq!(mapped_range_seq[0].len, 50);
    assert_eq!(mapped_range_seq[1].start, 2005);
    assert_eq!(mapped_range_seq[1].len, 5);
    assert_eq!(mapped_range_seq[2].start, 95);
    assert_eq!(mapped_range_seq[2].len, 5);
    assert_eq!(mapped_range_seq[3].start, 150);
    assert_eq!(mapped_range_seq[3].len, 10);
}

#[test]
fn test_multi_map_superset() {
    let map_item_a1 = MapItem {
        dst_range_start: 1000,
        src_range_start: 100,
        range_len: 50,
    };
    let map_item_a2 = MapItem {
        dst_range_start: 2000,
        src_range_start: 85,
        range_len: 10,
    };
    let map_a = vec![map_item_a1.clone(), map_item_a2.clone()];
    let map_item_b1 = MapItem {
        dst_range_start: 3007,
        src_range_start: 2007,
        range_len: 10,
    };
    let map_b = vec![map_item_b1.clone()];
    let map_seq = vec![map_a.clone(), map_b.clone()];
    let range = Range {
        start: 90, 
        len: 70, 
    };
    let range_seq = vec![range.clone()];
    let mapped_range_seq = apply_map_seq_to_range_seq(&map_seq, &range_seq);
    assert_eq!(mapped_range_seq.len(), 5);
    assert_eq!(mapped_range_seq[0].start, 1000);
    assert_eq!(mapped_range_seq[0].len, 50);
    assert_eq!(mapped_range_seq[1].start, 3007);
    assert_eq!(mapped_range_seq[1].len, 3);
    assert_eq!(mapped_range_seq[2].start, 2005);
    assert_eq!(mapped_range_seq[2].len, 2);
    assert_eq!(mapped_range_seq[3].start, 95);
    assert_eq!(mapped_range_seq[3].len, 5);
    assert_eq!(mapped_range_seq[4].start, 150);
    assert_eq!(mapped_range_seq[4].len, 10);
}