use std::cmp::{max, min};
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let filename = env::args().nth(1).unwrap_or(String::from("input.txt"));
    let parsed_lines = read_file_to_vec(filename);
    let part_1_answer = part1(&parsed_lines);
    let part_2_answer = part2(&parsed_lines);
    println!("Part 1: {}", part_1_answer);
    println!("Part 2: {}", part_2_answer);
}

fn read_file_to_vec(filename: String) -> Vec<String> {
    println!("Reading file {}", filename);
    let file = fs::File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let mut parsed_lines: Vec<String> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        parsed_lines.push(line.unwrap());
    }
    parsed_lines
}

fn part1(lines: &Vec<String>) -> u64 {
    let crab_positions: Vec<u64> = lines
        .iter()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let mut unique_sorted_positions = crab_positions.clone();
    unique_sorted_positions.sort();
    unique_sorted_positions.dedup();
    // arbitrary big number
    let mut best_position_cost = 2 << 32;
    for position_to_test in unique_sorted_positions {
        let mut cost = 0;
        for j in 0..crab_positions.len() {
            let current_item = crab_positions[j];
            cost += max(position_to_test, current_item) - min(position_to_test, current_item);
        }
        if cost < best_position_cost {
            best_position_cost = cost;
        }
    }
    best_position_cost
}

fn part2(lines: &Vec<String>) -> u64 {
    let crab_positions: Vec<u64> = lines
        .iter()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let mut unique_sorted_positions = crab_positions.clone();
    unique_sorted_positions.sort();
    unique_sorted_positions.dedup();
    // arbitrary big number
    let mut best_position_cost = 2 << 32;
    let first = unique_sorted_positions.iter().next().unwrap().clone();
    let last = unique_sorted_positions.iter().last().unwrap().clone();
    for position_to_test in first..=last {
        let mut cost = 0;
        for j in 0..crab_positions.len() {
            let current_item = crab_positions[j];
            let high = max(position_to_test.clone(), current_item);
            let low = min(position_to_test.clone(), current_item);
            let range = high - low;
            // Formula for consecutive number sum. (1..range).sum::<u64>() also works fine here
            cost += (range as f64 / 2_f64 * (1 + range) as f64) as u64;
        }
        if cost < best_position_cost {
            best_position_cost = cost;
        }
    }
    best_position_cost
}

#[test]
fn test_part1() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(37, part1(&sample_data));
}

#[test]
fn test_part2() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(66, (1..=11).sum());
    assert_eq!(0, (1..=0).sum());
    assert_eq!(1, (1..=1).sum());
    assert_eq!(168, part2(&sample_data));
}
