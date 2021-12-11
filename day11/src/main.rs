use std::collections::HashSet;
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

fn part1(lines: &Vec<String>) -> u64 {
    run_model(lines, 100)
}

fn run_model(lines: &Vec<String>, iters: usize) -> u64 {
    let mut grid = to_grid(lines);
    let mut total_flashers: u64 = 0;
    for _ in 0..iters {
        for x in 0..10 {
            for y in 0..10 {
                grid[x][y] += 1;
            }
        }
        let mut flashers = HashSet::new();
        mark_flashes(&mut grid, &mut flashers);
        for (j, k) in &flashers {
            grid[*j][*k] = 0;
        }
        total_flashers += flashers.len() as u64;
    }
    total_flashers
}

fn part2(lines: &Vec<String>) -> u64 {
    let mut i = 0;
    let mut grid = to_grid(lines);
    loop {
        i += 1;
        for x in 0..10 {
            for y in 0..10 {
                grid[x][y] += 1;
            }
        }
        let mut flashers = HashSet::new();
        mark_flashes(&mut grid, &mut flashers);
        for (j, k) in &flashers {
            grid[*j][*k] = 0;
        }

        if flashers.len() == 100 {
            return i;
        }
    }
}

fn to_grid(lines: &Vec<String>) -> [[u64; 10]; 10] {
    let mut grid = [[0_u64; 10]; 10];
    for (x, line) in lines.iter().enumerate() {
        for (y, digit) in line.chars().enumerate() {
            grid[x][y] = digit.to_digit(10).unwrap() as u64;
        }
    }
    grid
}

fn mark_flashes(grid: &mut [[u64; 10]; 10], flashers: &mut HashSet<(usize, usize)>) {
    for x in 0..10 {
        for y in 0..10 {
            mark_if_flashing(x, y, grid, flashers);
        }
    }
}

fn mark_if_flashing(
    x: usize,
    y: usize,
    grid: &mut [[u64; 10]; 10],
    flashers: &mut HashSet<(usize, usize)>,
) {
    let octopus_level = grid[x][y];
    let coords = (x, y);
    if octopus_level > 9 && !flashers.contains(&coords) {
        flashers.insert(coords);
        for (i, j) in [
            (-1_i8, -1_i8),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            let neighbor_x = x as i8 + i;
            let neighbor_y = y as i8 + j;
            if neighbor_x >= 0 && neighbor_x < 10 && neighbor_y >= 0 && neighbor_y < 10 {
                grid[neighbor_x as usize][neighbor_y as usize] += 1;
                mark_if_flashing(neighbor_x as usize, neighbor_y as usize, grid, flashers);
            }
        }
    }
}

#[test]
fn test_part1() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(35, run_model(&sample_data, 2));
    assert_eq!(204, run_model(&sample_data, 10));
    assert_eq!(1656, part1(&sample_data));
}

#[test]
fn test_part2() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(195, part2(&sample_data));
}
