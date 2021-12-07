use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::ops::Sub;

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
    run_laternfish_simulation(lines, 80)
}

fn part2(lines: &Vec<String>) -> u64 {
    run_laternfish_simulation(lines, 256)
}

fn run_laternfish_simulation(lines: &Vec<String>, num_days: usize) -> u64 {
    let initial_state: Vec<u64> = lines
        .iter()
        .next()
        .unwrap()
        .split(",")
        .map(|f| f.parse().unwrap())
        .collect();
    let mut counts_by_time_left: HashMap<u64, u64> = HashMap::new();
    let mut total_fish: u64 = initial_state.len() as u64;
    for state in initial_state {
        *counts_by_time_left.entry(state).or_insert(0) += 1;
    }
    let mut adds_by_day = vec![0; num_days + 1];
    for day in 1..=num_days {
        let spawning_timers: u64 = ((day - 1) % 7) as u64;
        let num_to_ignore = if day > 2 { adds_by_day[day - 2] } else { 0 };
        let count_at_time_left = counts_by_time_left
            .get(&spawning_timers)
            .unwrap_or(&0)
            .clone();
        let count_at_timer = count_at_time_left.sub(num_to_ignore);
        let position_to_add_at = (spawning_timers + 2) % 7;
        *counts_by_time_left.entry(position_to_add_at).or_insert(0) += count_at_timer;
        total_fish += count_at_timer;
        adds_by_day.insert(day, count_at_timer.clone());
    }
    total_fish
}

#[test]
fn test_part1() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(26, run_laternfish_simulation(&sample_data, 18));
    assert_eq!(5934, part1(&sample_data));
}

#[test]
fn test_part2() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(26984457539, part2(&sample_data));
}
