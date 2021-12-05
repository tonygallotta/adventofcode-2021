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
    overlapping_points(lines, false)
}

fn overlapping_points(lines: &Vec<String>, include_diagonal: bool) -> u32 {
    let mut overlapping_points = HashSet::new();
    let mut seen_points = HashSet::new();
    for line in lines {
        let mut parts = line.split_ascii_whitespace();
        let mut point1_parts = parts.next().unwrap().split(",");
        let start_point: (u32, u32) = (
            point1_parts.next().unwrap().parse().unwrap(),
            point1_parts.next().unwrap().parse().unwrap(),
        );
        parts.next();
        let mut point2_parts = parts.next().unwrap().split(",");
        let end_point: (u32, u32) = (
            point2_parts.next().unwrap().parse().unwrap(),
            point2_parts.next().unwrap().parse().unwrap(),
        );
        if include_diagonal || start_point.0 == end_point.0 || start_point.1 == end_point.1 {
            let points_in_line = all_points_in_line(&start_point, &end_point);
            for point in points_in_line {
                if seen_points.contains(&point) {
                    overlapping_points.insert(point.clone());
                }
                seen_points.insert(point.clone());
            }
        }
    }
    overlapping_points.len() as u32
}

fn all_points_in_line(start_point: &(u32, u32), end_point: &(u32, u32)) -> Vec<(u32, u32)> {
    let mut points = Vec::new();
    let slope_x = (end_point.0 as i32 - start_point.0 as i32).signum();
    let slope_y = (end_point.1 as i32 - start_point.1 as i32).signum();
    let mut x = start_point.0;
    let mut y = start_point.1;
    while &(x, y) != end_point {
        points.push((x, y));
        x = (x as i32 + slope_x) as u32;
        y = (y as i32 + slope_y) as u32;
    }
    points.push((x, y));
    points
}

fn part2(lines: &Vec<String>) -> u32 {
    overlapping_points(lines, true)
}

#[test]
fn test_part1() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(5, part1(&sample_data));
}

#[test]
fn test_part2() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(
        vec![(1, 1), (2, 2), (3, 3)],
        all_points_in_line(&(1, 1), &(3, 3))
    );
    assert_eq!(
        vec![(9, 7), (8, 8), (7, 9)],
        all_points_in_line(&(9, 7), &(7, 9))
    );
    assert_eq!(12, part2(&sample_data));
}
