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
    let mut unique_segment_count = 0;
    // # of segments in 1, 4, 7, 8
    let unique_segment_sizes: Vec<usize> = vec![2, 4, 3, 7];
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
    lines.iter().map(|l| decode_segments(l)).sum()
}

// Returns a vector of size 7 where the position of the element corresponds to a-g and the value
// indicates what it is actually mapped to.
fn decode_segments(line: &String) -> u64 {
    let mut known_segments = vec!["x"; 10];
    let mut input_output_parts = line.split("|");
    let observations: Vec<&str> = input_output_parts
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .collect();
    let outputs: Vec<&str> = input_output_parts
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .collect();
    const ONE_LENGTH: usize = 2;
    const FOUR_LENGTH: usize = 4;
    const SEVEN_LENGTH: usize = 3;
    const EIGHT_LENGTH: usize = 7;
    for observation in &observations {
        // let chars = observation.chars().clone();
        match observation.len() {
            ONE_LENGTH => {
                // println!("Got a 1! {}", observation);
                known_segments[1] = observation.clone();
            }
            FOUR_LENGTH => {
                // println!("Got a 4! {}", observation);
                known_segments[4] = observation.clone();
            }
            SEVEN_LENGTH => {
                // println!("Got a 7! {}", observation);
                known_segments[7] = observation.clone();
            }
            EIGHT_LENGTH => {
                // println!("Got an 8! {}", observation);
                known_segments[8] = observation.clone();
            }
            _ => {}
        }
    }
    // A 9 contains all the segments of a 4, plus two more, so we can determine that now
    // Notes on remaining digits
    // 0 -> 6 AND contains both segments of 1
    let chars_in_one: Vec<char> = known_segments[1].chars().collect();
    let chars_in_four: Vec<char> = known_segments[4].chars().collect();
    for observation in &observations {
        if observation.len() == 6 {
            let chars_in_observation: Vec<char> = observation.chars().collect();
            if chars_in_four
                .iter()
                .all(|c| chars_in_observation.contains(c))
            {
                known_segments[9] = observation.clone();
            }
        }
    }
    for observation in &observations {
        if observation.len() == 6 && observation != &known_segments[9] {
            let chars_in_observation: Vec<char> = observation.chars().collect();
            if chars_in_one
                .iter()
                .all(|c| chars_in_observation.contains(c))
            {
                known_segments[0] = observation.clone();
            }
        }
    }
    // 6 -> 6 is the other one of length 6
    for observation in &observations {
        if observation.len() == 6
            && observation != &known_segments[0]
            && observation != &known_segments[9]
        {
            known_segments[6] = observation.clone();
        }
    }
    // 3 -> 5 AND contains all segments of 7
    // 5 -> 5 Six contains all segments of 5 (2nd to last to determine)
    let chars_in_seven: Vec<char> = known_segments[1].chars().collect();
    let chars_in_six: Vec<char> = known_segments[6].chars().collect();
    for observation in &observations {
        if observation.len() == 5 {
            let chars_in_observation: Vec<char> = observation.chars().collect();
            if chars_in_seven
                .iter()
                .all(|c| chars_in_observation.contains(c))
            {
                known_segments[3] = observation.clone();
            } else if chars_in_observation
                .iter()
                .all(|c| chars_in_six.contains(c))
            {
                known_segments[5] = observation.clone();
            }
        }
    }
    // 2 -> 5 the last one
    for observation in &observations {
        if observation.len() == 5
            && observation != &known_segments[3]
            && observation != &known_segments[5]
        {
            known_segments[2] = observation.clone();
        }
    }
    for (i, c) in known_segments.iter().enumerate() {
        println!("Determined {}=>{}", i, c);
    }
    let mut output_value: u64 = 0;
    for (i, output) in outputs.iter().enumerate() {
        println!("Checking output {}", output);
        let digit = known_segments
            .iter()
            .position(|&v| {
                v.chars().all(|c| output.contains(c)) && output.chars().all(|c| v.contains(c))
            })
            .unwrap();
        output_value += 10_u64.pow(3_u32.sub(i as u32)) * digit as u64;
    }
    output_value
}

#[test]
fn test_part1() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(26, part1(&sample_data));
}

#[test]
fn test_part2() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    // assert_eq!(5353, decode_segments(&String::from(
    //     "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
    // )));
    assert_eq!(5092, decode_segments(&String::from(
        "dg fadgceb dacbef agfeb gcdbef edcbf gdf ecgd cgbadf defbg | bedcf bgdfac cbfedg abfeg",
    )));
    assert_eq!(61229, part2(&sample_data));
}
