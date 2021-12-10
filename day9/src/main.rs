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
    for (_, line) in reader.lines().enumerate() {
        parsed_lines.push(line.unwrap());
    }
    parsed_lines
}

fn part1(lines: &Vec<String>) -> u64 {
    let height_map = to_2d(lines);
    let mut risk = 0;
    for i in 0..height_map.len() {
        for j in 0..height_map[0].len() {
            if smaller_than_all_neighbors(i, j, &height_map) {
                risk += height_map[i][j] + 1;
            }
        }
    }
    risk
}

fn part2(lines: &Vec<String>) -> u64 {
    let height_map = to_2d(lines);
    let mut basin_sizes = Vec::new();
    for i in 0..height_map.len() {
        for j in 0..height_map[0].len() {
            if smaller_than_all_neighbors(i, j, &height_map) {
                basin_sizes.push(basin_size(i, j, &height_map));
            }
        }
    }
    basin_sizes.sort();
    basin_sizes.reverse();
    basin_sizes[0..3]
        .iter()
        .copied()
        .reduce(|a, b| a * b)
        .unwrap()
        .clone()
}

fn to_2d(lines: &Vec<String>) -> Vec<Vec<u64>> {
    let mut as_vec = Vec::new();
    for line in lines {
        as_vec.push(
            line.chars()
                .map(|c| u64::from(c.to_digit(10).unwrap()))
                .collect(),
        );
    }
    as_vec
}

fn smaller_than_all_neighbors(i: usize, j: usize, matrix: &Vec<Vec<u64>>) -> bool {
    let total_rows = matrix.len();
    let total_cols = matrix[0].len();
    let row_neighbors_to_check = if i == 0 {
        vec![i + 1]
    } else if i == total_rows - 1 {
        vec![i - 1]
    } else {
        vec![i - 1, i + 1]
    };
    let col_neighbors_to_check = if j == 0 {
        vec![j + 1]
    } else if j == total_cols - 1 {
        vec![j - 1]
    } else {
        vec![j - 1, j + 1]
    };
    let current_point = matrix[i][j];
    row_neighbors_to_check
        .iter()
        .all(|n| matrix[*n][j] > current_point)
        && col_neighbors_to_check
            .iter()
            .all(|n| matrix[i][*n] > current_point)
}

fn basin_size(i: usize, j: usize, matrix: &Vec<Vec<u64>>) -> u64 {
    let mut visited = HashSet::new();
    visited.insert((i, j));
    return dfs(i, j, matrix, &mut visited);
}

fn dfs(i: usize, j: usize, matrix: &Vec<Vec<u64>>, visited: &mut HashSet<(usize, usize)>) -> u64 {
    let total_rows = matrix.len();
    let total_cols = matrix[0].len();
    let row_neighbors_to_check = if i == 0 {
        vec![i + 1]
    } else if i == total_rows - 1 {
        vec![i - 1]
    } else {
        vec![i - 1, i + 1]
    };
    let col_neighbors_to_check = if j == 0 {
        vec![j + 1]
    } else if j == total_cols - 1 {
        vec![j - 1]
    } else {
        vec![j - 1, j + 1]
    };
    let mut neighbor_size = 1;
    for x in &row_neighbors_to_check {
        if !visited.contains(&(*x, j)) && matrix[*x][j] != 9 {
            visited.insert((*x, j));
            neighbor_size += dfs(*x, j, matrix, visited);
        }
    }
    for y in &col_neighbors_to_check {
        if !visited.contains(&(i, *y)) && matrix[i][*y] != 9 {
            visited.insert((i, *y));
            neighbor_size += dfs(i, *y, matrix, visited);
        }
    }
    return neighbor_size;
}

#[test]
fn test_part1() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    let heatmap = to_2d(&sample_data);
    assert_eq!(true, smaller_than_all_neighbors(0, 1, &heatmap));
    assert_eq!(false, smaller_than_all_neighbors(0, 0, &heatmap));
    assert_eq!(15, part1(&sample_data));
}

#[test]
fn test_part2() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    let heatmap = to_2d(&sample_data);
    assert_eq!(3, basin_size(0, 1, &heatmap));
    assert_eq!(1134, part2(&sample_data));
}
