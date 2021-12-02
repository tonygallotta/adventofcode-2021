use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Coords {
    horizontal: u32,
    depth: u32,
}

fn main() {
    let filename = env::args().nth(1).unwrap_or(String::from("input.txt"));
    let parsed_lines = read_file_to_vec(filename);
    let part_1_answer = part1(&parsed_lines);
    let part_2_answer = part2(&parsed_lines);
    println!(
        "Part 1: {:?} -> {}",
        part_1_answer,
        part_1_answer.depth * part_1_answer.horizontal
    );
    println!(
        "Part 2: {:?} -> {}",
        part_2_answer,
        part_2_answer.depth * part_2_answer.horizontal
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

fn part1(lines: &Vec<String>) -> Coords {
    let mut horizontal = 0;
    let mut depth = 0;
    for line in lines {
        let mut parts = line.split(" ");
        let command = parts.next().unwrap();
        let amount: u32 = parts.next().unwrap().parse().unwrap();
        match command {
            "forward" => horizontal = horizontal + amount,
            "down" => depth = depth + amount,
            "up" => depth = depth - amount,
            _ => {}
        }
    }
    Coords { horizontal, depth }
}

fn part2(lines: &Vec<String>) -> Coords {
    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;
    for line in lines {
        let mut parts = line.split(" ");
        let command = parts.next().unwrap();
        let amount: u32 = parts.next().unwrap().parse().unwrap();
        match command {
            "forward" => {
                horizontal = horizontal + amount;
                depth = depth + aim * amount;
            }
            "down" => aim = aim + amount,
            "up" => aim = aim - amount,
            _ => {}
        }
    }
    Coords { horizontal, depth }
}
