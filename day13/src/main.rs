use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::ops::Sub;
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

fn part1(lines: &Vec<String>) -> usize {
    let mut points: HashSet<(usize, usize)> = HashSet::new();
    let mut i = 0;
    while !lines[i].is_empty() {
        let mut point_parts = lines[i].split(',');
        points.insert((
            point_parts.next().unwrap().parse().unwrap(),
            point_parts.next().unwrap().parse().unwrap(),
        ));
        i += 1;
    }
    let mut fold_instruction = lines[i + 1]
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .split('=');
    let fold_direction = fold_instruction.next().unwrap().chars().next().unwrap();
    let fold_line: usize = fold_instruction.next().unwrap().parse().unwrap();

    println!("fold along {} = {}", fold_direction, fold_line);
    let x_max = points.iter().map(|p| p.0).max().unwrap();
    let y_max = points.iter().map(|p| p.1).max().unwrap();
    fold_grid(&mut points, fold_direction, fold_line, x_max, y_max);
    points.len()
}

fn fold_grid(
    points: &mut HashSet<(usize, usize)>,
    fold_direction: char,
    fold_line: usize,
    x_max: usize,
    y_max: usize,
) {
    let mut points_to_add = HashSet::new();
    let mut points_to_remove = HashSet::new();
    if fold_direction == 'y' {
        for y in fold_line + 1..=y_max {
            for x in 0..=x_max {
                if points.contains(&(x, y)) {
                    let fold_to = (x, y_max.sub(y) as usize);
                    points_to_add.insert(fold_to);
                    points_to_remove.insert((x, y));
                }
            }
        }
    } else {
        for x in fold_line + 1..=x_max {
            for y in 0..=y_max {
                if points.contains(&(x, y)) {
                    let fold_to = (x_max.sub(x) as usize, y);
                    points_to_add.insert(fold_to);
                    points_to_remove.insert((x, y));
                }
            }
        }
    }
    for p in points_to_remove {
        points.remove(&p);
    }
    for p in points_to_add {
        points.insert(p);
    }
}

fn part2(lines: &Vec<String>) -> usize {
    let mut points: HashSet<(usize, usize)> = HashSet::new();
    let mut i = 0;
    while !lines[i].is_empty() {
        let mut point_parts = lines[i].split(',');
        points.insert((
            point_parts.next().unwrap().parse().unwrap(),
            point_parts.next().unwrap().parse().unwrap(),
        ));
        i += 1;
    }
    i += 1;
    let mut x_max = points.iter().map(|p| p.0).max().unwrap();
    let mut y_max = points.iter().map(|p| p.1).max().unwrap();
    while i < lines.len() {
        let mut fold_instruction = lines[i].split_ascii_whitespace().last().unwrap().split('=');
        let fold_direction = fold_instruction.next().unwrap().chars().next().unwrap();
        let fold_line: usize = fold_instruction.next().unwrap().parse().unwrap();

        println!("fold along {} = {}", fold_direction, fold_line);
        fold_grid(&mut points, fold_direction, fold_line, x_max, y_max);
        if fold_direction == 'y' {
            y_max = fold_line - 1;
        } else {
            x_max = fold_line - 1;
        }
        i += 1;
    }
    for y in 0..=y_max {
        for x in 0..=x_max {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    points.len()
}

#[test]
fn test_part1() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(17, part1(&sample_data));
}

#[test]
fn test_part2() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(16, part2(&sample_data));
}
