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
    println!("Part 1: {}", part_2_answer);
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

fn part2(lines: &Vec<String>) -> u32 {
    let total_bits = lines[0].len();
    let mut one_counts: Vec<u32> = vec![0; total_bits];
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                one_counts[i] += 1;
            }
        }
    }
    let mut majority_threshold: u32 = (lines.len() as f64 / 2 as f64).ceil() as u32;
    let mut kept_indices = HashSet::new();
    let mut oxygen_generator_rating: Option<u32> = None;
    let mut co2_scrubber_rating: Option<u32> = None;
    for i in 0..lines.len() {
        kept_indices.insert(i);
    }
    let mut iters = 0;
    println!("Starting the while loop!");
    while oxygen_generator_rating == None && iters < 10000 {
        for current_index in 0..total_bits {
            let mut ones_at_index = 0;
            // println!(
            //     "Checking bit index {} in {} entries, majority threshold is {}",
            //     current_index,
            //     kept_indices.len(),
            //     majority_threshold
            // );
            for i in &kept_indices {
                let line = &lines[i.clone()];
                if line.chars().nth(current_index).unwrap() == '1' {
                    ones_at_index += 1;
                }
            }
            let mut indices_to_remove = HashSet::new();
            if ones_at_index >= majority_threshold {
                // println!("The bit at {} must be a 1", current_index);
                for i in &kept_indices {
                    if lines[i.clone()].chars().nth(current_index).unwrap() != '1' {
                        indices_to_remove.insert(i.clone());
                    }
                }
            } else {
                // println!("The bit at {} must be a 1", current_index);
                for i in &kept_indices {
                    if lines[i.clone()].chars().nth(current_index).unwrap() == '1' {
                        indices_to_remove.insert(i.clone());
                    }
                }
            }
            for i in indices_to_remove {
                // println!("Removing {}-{}", i, lines[i]);
                kept_indices.remove(&i);
            }
            majority_threshold = (kept_indices.len() as f64 / 2_f64).ceil() as u32;
            if kept_indices.len() == 1 {
                let only_index = kept_indices.iter().next().unwrap().clone();
                oxygen_generator_rating =
                    Option::Some(u32::from_str_radix(&lines[only_index], 2).unwrap());
                println!("Found it! at index {}: {}", only_index, &lines[only_index]);
            }
            // println!(
            //     "{} remaining after iteration {} on bit {}",
            //     kept_indices.len(),
            //     iters,
            //     current_index
            // );
        }
        iters += 1;
    }

    for i in 0..lines.len() {
        kept_indices.insert(i);
    }
    let mut iters = 0;
    println!("Starting the 2nd while loop!");
    while co2_scrubber_rating == None && iters < 10000 {
        for current_index in 0..total_bits {
            let mut ones_at_index = 0;
            // println!(
            //     "Checking bit index {} in {} entries, majority threshold is {}",
            //     current_index,
            //     kept_indices.len(),
            //     majority_threshold
            // );
            for i in &kept_indices {
                let line = &lines[i.clone()];
                if line.chars().nth(current_index).unwrap() == '1' {
                    ones_at_index += 1;
                }
            }
            let mut indices_to_remove = HashSet::new();
            if ones_at_index >= majority_threshold {
                // println!("The bit at {} must be a 1", current_index);
                for i in &kept_indices {
                    if lines[i.clone()].chars().nth(current_index).unwrap() != '0' {
                        indices_to_remove.insert(i.clone());
                    }
                }
            } else {
                // println!("The bit at {} must be a 1", current_index);
                for i in &kept_indices {
                    if lines[i.clone()].chars().nth(current_index).unwrap() == '0' {
                        indices_to_remove.insert(i.clone());
                    }
                }
            }
            for i in indices_to_remove {
                // println!("Removing {}-{}", i, lines[i]);
                kept_indices.remove(&i);
            }
            majority_threshold = (kept_indices.len() as f64 / 2_f64).ceil() as u32;
            if kept_indices.len() == 1 {
                let only_index = kept_indices.iter().next().unwrap().clone();
                co2_scrubber_rating =
                    Option::Some(u32::from_str_radix(&lines[only_index], 2).unwrap());
                println!("Found it! at index {}: {}", only_index, &lines[only_index]);
            }
            // println!(
            //     "{} remaining after iteration {} on bit {}",
            //     kept_indices.len(),
            //     iters,
            //     current_index
            // );
        }
        iters += 1;
    }
    oxygen_generator_rating.unwrap() * co2_scrubber_rating.unwrap()
}

#[test]
fn test_part1() {
    let sample_data = read_file_to_vec(String::from("test_1.txt"));
    assert_eq!(198, part1(&sample_data));
}

#[test]
fn test_part2() {
    let sample_data = read_file_to_vec(String::from("test_1.txt"));
    assert_eq!(230, part2(&sample_data));
}
