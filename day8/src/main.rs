use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::ops::Sub;
use std::time::Instant;

const ONE_LENGTH: usize = 2;
const FOUR_LENGTH: usize = 4;
const SEVEN_LENGTH: usize = 3;
const EIGHT_LENGTH: usize = 7;

fn main() {
    let now = Instant::now();
    let filename = env::args().nth(1).unwrap_or(String::from("input.txt"));
    let parsed_lines = read_file_to_vec(filename);
    let part_1_answer = part1(&parsed_lines);
    println!(
        "Part 1 ({}ms): {}",
        now.elapsed().as_millis(),
        part_1_answer
    );
    let part_2_answer = part2(&parsed_lines);
    println!(
        "Part 2 ({}ms): {}",
        now.elapsed().as_millis(),
        part_2_answer
    );
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
    let mut unique_segment_count = 0;
    // # of segments in 1, 4, 7, 8
    let unique_segment_sizes: HashSet<usize> =
        HashSet::from([ONE_LENGTH, FOUR_LENGTH, SEVEN_LENGTH, EIGHT_LENGTH]);
    for line in lines {
        let output_part = line.split(" | ").nth(1).unwrap();
        unique_segment_count += output_part
            .split_ascii_whitespace()
            .filter(|output| unique_segment_sizes.contains(&output.len()))
            .count();
    }
    unique_segment_count as u64
}

fn part2(lines: &Vec<String>) -> u64 {
    lines.iter().map(|l| determine_output(l)).sum()
}

// Returns the output value for the 7-segment display the line represents.
fn determine_output(line: &String) -> u64 {
    let mut identified_signal_patterns = vec!["x"; 10];
    let mut observation_output_parts = line.split("|");
    let observations: Vec<&str> = observation_output_parts
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .collect();
    let outputs: Vec<&str> = observation_output_parts
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .collect();
    let mut undetermined_observations = HashSet::new();
    // Determine the easy digits: 1, 4, 7 and 8
    for (i, observation) in observations.iter().enumerate() {
        match observation.len() {
            ONE_LENGTH => {
                identified_signal_patterns[1] = observation.clone();
            }
            FOUR_LENGTH => {
                identified_signal_patterns[4] = observation.clone();
            }
            SEVEN_LENGTH => {
                identified_signal_patterns[7] = observation.clone();
            }
            EIGHT_LENGTH => {
                identified_signal_patterns[8] = observation.clone();
            }
            _ => {
                undetermined_observations.insert(i);
            }
        }
    }
    // Determine the digits with 6 segments.
    // 9 uniquely contains all the segments of a 4
    // 0 uniquely contains all segments of 1
    // 6 is the other one
    for observation_index in undetermined_observations.clone().iter() {
        let observation = observations[*observation_index];
        if observation.len() == 6 {
            if contains_all_chars(observation, identified_signal_patterns[4]) {
                identified_signal_patterns[9] = observation.clone();
                undetermined_observations.remove(&observation_index.clone());
            } else if contains_all_chars(observation, identified_signal_patterns[1]) {
                identified_signal_patterns[0] = observation.clone();
                undetermined_observations.remove(&observation_index.clone());
            } else {
                identified_signal_patterns[6] = observation.clone();
                undetermined_observations.remove(&observation_index.clone());
            }
        }
    }
    // All remaining digits have 5 segments lit
    // 3 contains all segments of 1
    // 6 contains all segments of 5
    for observation_index in undetermined_observations.clone().iter() {
        let observation = observations[*observation_index];
        if contains_all_chars(observation, identified_signal_patterns[7]) {
            identified_signal_patterns[3] = observation.clone();
            undetermined_observations.remove(&observation_index.clone());
        } else if contains_all_chars(identified_signal_patterns[6], observation) {
            identified_signal_patterns[5] = observation.clone();
            undetermined_observations.remove(&observation_index.clone());
        }
    }
    // 2 is the last undetermined digit
    identified_signal_patterns[2] =
        observations[undetermined_observations.iter().next().unwrap().clone()];

    // Now we know all the unique signal patterns and can decode the output
    let mut output_value: u64 = 0;
    for (i, output) in outputs.iter().enumerate() {
        let digit = identified_signal_patterns
            .iter()
            .position(|&v| contains_all_chars(v, output) && contains_all_chars(output, v))
            .unwrap();
        output_value += 10_u64.pow(3_u32.sub(i as u32)) * digit as u64;
    }
    output_value
}

fn contains_all_chars(container: &str, contained: &str) -> bool {
    contained.chars().all(|c| container.contains(c))
}

#[test]
fn test_part1() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(26, part1(&sample_data));
}

#[test]
fn test_contains_all_chars() {
    assert!(contains_all_chars("abcd", "bac"));
    assert!(!contains_all_chars("bac", "abcd"));
}

#[test]
fn test_part2() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(
        5353,
        determine_output(&String::from(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        ))
    );
    assert_eq!(5092, determine_output(&String::from(
        "dg fadgceb dacbef agfeb gcdbef edcbf gdf ecgd cgbadf defbg | bedcf bgdfac cbfedg abfeg",
    )));
    assert_eq!(61229, part2(&sample_data));
}
