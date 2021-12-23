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
    for line in reader.lines() {
        parsed_lines.push(line.unwrap());
    }
    parsed_lines
}

fn part1(lines: &Vec<String>) -> u32 {
    let mut player_1_pos = lines[0].chars().last().unwrap().to_digit(10).unwrap();
    let mut player_2_pos = lines[1].chars().last().unwrap().to_digit(10).unwrap();
    let mut dice = 1;
    let mut player_1_score = 0;
    let mut player_2_score = 0;
    let mut rolls = 0;
    loop {
        for _ in 0..3 {
            player_1_pos += dice;
            dice = dice % 100 + 1;
            rolls += 1;
        }
        let current_score = player_1_pos % 10;
        player_1_score += if current_score == 0 {
            10
        } else {
            current_score
        };
        if player_1_score >= 1000 {
            return rolls * player_2_score;
        }

        for _ in 0..3 {
            player_2_pos += dice;
            dice = dice % 100 + 1;
            rolls += 1;
        }
        let current_score = player_2_pos % 10;
        player_2_score += if current_score == 0 {
            10
        } else {
            current_score
        };
        if player_2_score >= 1000 {
            return rolls * player_1_score;
        }
    }
}

fn part2(lines: &Vec<String>) -> usize {
    lines.len()
}

#[test]
fn test_part1() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(739785, part1(&sample_data));
}

#[test]
fn test_part2() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(0, part2(&sample_data));
}
