use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::time::Instant;

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
    let part_2_answer = part2(&parsed_lines, 20);
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
    for line in reader.lines() {
        parsed_lines.push(line.unwrap());
    }
    parsed_lines
}

fn part1(lines: &Vec<String>) -> u32 {
    let pair_insertion_rules: HashMap<String, String> = lines[2..]
        .iter()
        .map(|l| l.split_once(" -> ").unwrap().clone())
        .map(|p| (p.0.to_string(), p.1.to_string()))
        .collect();
    let mut polymer_template = lines[0].clone();
    for _ in 0..10 {
        polymer_template = do_insertions(&polymer_template, &pair_insertion_rules);
    }
    println!("Final length was {}", polymer_template.len());
    let mut counts_by_char: HashMap<char, u32> = HashMap::new();
    for c in polymer_template.chars() {
        *counts_by_char.entry(c).or_insert(0) += 1;
    }
    let mut counts: Vec<u32> = counts_by_char.iter().map(|(_, v)| v.clone()).collect();
    counts.sort();
    counts.last().unwrap() - counts.first().unwrap()
}

fn do_insertions(
    polymer_template: &String,
    pair_insertion_rules: &HashMap<String, String>,
) -> String {
    let mut new_template = String::new();
    let polymer_template_chars: Vec<char> = polymer_template.chars().collect();
    for i in 0..polymer_template.len() - 1 {
        let current_pair = polymer_template_chars
            .iter()
            .skip(i)
            .take(2)
            .collect::<String>();
        let mapped = pair_insertion_rules.get(&current_pair).unwrap();
        new_template.push(polymer_template_chars[i]);
        new_template.push_str(mapped);
    }
    new_template.push(*polymer_template_chars.last().unwrap());
    new_template
}

// Now we need to run 40 iterations, and generating the actual string would occupy TBs
fn part2(lines: &Vec<String>, half_iters: u32) -> u64 {
    let pair_insertion_rules: HashMap<String, String> = lines[2..]
        .iter()
        .map(|l| l.split_once(" -> ").unwrap().clone())
        .map(|p| (p.0.to_string(), p.1.to_string()))
        .collect();
    // Figure out what each pair expands to after 20 insertions
    let mut pair_char_counts_after_20: HashMap<String, HashMap<char, u64>> = HashMap::new();
    for pair in pair_insertion_rules.keys() {
        let mut expanded_pair = pair.clone();
        let mut counts_by_char: HashMap<char, u64> = HashMap::new();
        for _ in 0..half_iters {
            expanded_pair = do_insertions(&expanded_pair, &pair_insertion_rules);
        }
        // println!("{} expands to {}", pair, expanded_pair);
        for c in expanded_pair.chars() {
            *counts_by_char.entry(c).or_insert(0) += 1;
        }
        // Don't count the pair themselves
        *counts_by_char
            .entry(pair.chars().nth(0).unwrap())
            .or_insert(0) -= 1;
        *counts_by_char
            .entry(pair.chars().last().unwrap())
            .or_insert(0) -= 1;
        pair_char_counts_after_20.insert(pair.clone(), counts_by_char);
    }
    // Now do 20 insertions on the full template
    let mut polymer_template = lines[0].clone();
    for _ in 0..half_iters {
        polymer_template = do_insertions(&polymer_template, &pair_insertion_rules);
    }
    // println!("After {}: {}", half_iters, polymer_template);
    let polymer_template_chars: Vec<char> = polymer_template.chars().collect();
    let mut counts_by_char: HashMap<char, u64> = HashMap::new();
    for i in 0..polymer_template_chars.len() - 1 {
        // Increment the count for the current character
        let current_pair = polymer_template_chars
            .iter()
            .skip(i)
            .take(2)
            .collect::<String>();
        let counts_for_pair = pair_char_counts_after_20.get(&current_pair).unwrap();
        for (c, v) in counts_for_pair {
            // println!("In expansion of {}, {} occurs {} times", current_pair, c, v);
            *counts_by_char.entry(c.clone()).or_insert(0) += v;
        }
        *counts_by_char.entry(polymer_template_chars[i]).or_insert(0) += 1;
    }
    *counts_by_char
        .entry(*polymer_template_chars.last().unwrap())
        .or_insert(0) += 1;
    let mut counts: Vec<u64> = counts_by_char.iter().map(|(_, v)| v.clone()).collect();
    println!("B occurs {} times", counts_by_char.get(&'B').unwrap());
    println!("H occurs {} times", counts_by_char.get(&'H').unwrap());
    counts.sort();
    counts.last().unwrap() - counts.first().unwrap()
}

#[test]
fn test_part1() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(1588, part1(&sample_data));
}

#[test]
fn test_part2() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(18, part2(&sample_data, 2));
    assert_eq!(1588, part2(&sample_data, 5));
    assert_eq!(2188189693529, part2(&sample_data, 20));
}
