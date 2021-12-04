use std::collections::HashSet;
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

fn part1(lines: &Vec<String>) -> u32 {
    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;
    let total_bits = lines[0].len();
    let mut one_counts: Vec<u32> = vec![0; total_bits];
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                one_counts[i] += 1;
            }
        }
    }
    let majority_threshold: u32 = (lines.len() as u32 / 2 as u32) as u32;
    for (i, count) in one_counts.iter().enumerate() {
        let bit_position: u32 = (total_bits - i - 1) as u32;
        if count >= &majority_threshold {
            gamma_rate += 2_u32.pow(bit_position);
        } else {
            epsilon_rate += 2_u32.pow(bit_position);
        }
    }
    println!("gamma = {}, epsilon = {}", gamma_rate, epsilon_rate);
    gamma_rate * epsilon_rate
}

fn part2_recursive(lines: &Vec<String>) -> u32 {
    let oxygen_generator_rating = filter_part_2_recursive(lines.clone(), 0, false);
    let co2_scrubber_rating = filter_part_2_recursive(lines.clone(), 0, true);
    oxygen_generator_rating * co2_scrubber_rating
}

fn filter_part_2_recursive(lines: Vec<String>, position: usize, negate: bool) -> u32 {
    if lines.len() == 1 {
        return u32::from_str_radix(lines.iter().next().unwrap(), 2).unwrap();
    }
    let most_common_bit = if negate {
        !most_common_bit_at_position(&lines, position)
    } else {
        most_common_bit_at_position(&lines, position)
    };
    return filter_part_2_recursive(
        lines
            .iter()
            .filter(|l| {
                let bit_at_pos = l.chars().nth(position).unwrap();
                if most_common_bit {
                    bit_at_pos == '1'
                } else {
                    bit_at_pos == '0'
                }
            })
            .map(|l| l.clone())
            .collect(),
        position + 1,
        negate,
    );
}

// 1 is true, 0 is false
fn most_common_bit_at_position(lines: &Vec<String>, position: usize) -> bool {
    let majority_threshold = (lines.len() as f64 / 2_f64).ceil() as usize;
    lines
        .iter()
        .filter(|l| l.chars().nth(position).unwrap() == '1')
        .count()
        >= majority_threshold
}

#[test]
fn test_part1() {
    let sample_data = read_file_to_vec(String::from("test_1.txt"));
    assert_eq!(198, part1(&sample_data));
}

#[test]
fn test_part2() {
    let sample_data = read_file_to_vec(String::from("test_1.txt"));
    assert_eq!(230, part2_recursive(&sample_data));
}
