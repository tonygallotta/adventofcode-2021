use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Board(Vec<Vec<u32>>);

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

fn read_boards(lines: &Vec<String>) -> Vec<Board> {
    let mut current_board: Vec<Vec<u32>> = Vec::with_capacity(5);
    let mut boards = Vec::new();
    for i in 2..lines.len() {
        if lines[i].is_empty() {
            boards.push(Board(current_board.clone()));
            current_board = Vec::with_capacity(5);
            continue;
        }
        let mut current_row = Vec::with_capacity(5);
        for (_, value) in lines[i].split_ascii_whitespace().enumerate() {
            current_row.push(value.parse().unwrap());
        }
        current_board.push(current_row);
    }
    boards.push(Board(current_board.clone()));
    boards
}

fn part1(lines: &Vec<String>) -> u32 {
    let all_called_numbers: Vec<u32> = lines
        .iter()
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .map(|(_, l)| l.parse().unwrap())
        .collect();
    let boards = read_boards(&lines);
    for call_index in 4..all_called_numbers.len() {
        let currently_called = &all_called_numbers[0..=call_index];
        for board in &boards {
            for i in 0..=4 {
                if has_row_win(currently_called, &board, i)
                    || has_column_win(currently_called, &board, i)
                {
                    return calculate_unmarked_sum(&board, currently_called)
                        * all_called_numbers[call_index];
                }
            }
        }
    }
    0
}

fn part2(lines: &Vec<String>) -> u32 {
    let all_called_numbers: Vec<u32> = lines
        .iter()
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .map(|(_, l)| l.parse().unwrap())
        .collect();
    let boards = read_boards(&lines);
    let mut boards_with_wins = HashSet::new();
    for call_index in 4..all_called_numbers.len() {
        let currently_called = &all_called_numbers[0..=call_index];
        for (board_num, board) in boards.iter().enumerate() {
            for i in 0..=4 {
                if has_row_win(currently_called, &board, i)
                    || has_column_win(currently_called, &board, i)
                {
                    boards_with_wins.insert(board_num);
                    if boards_with_wins.len() == boards.len() {
                        return calculate_unmarked_sum(&board, currently_called)
                            * all_called_numbers[call_index];
                    }
                }
            }
        }
    }
    0
}

fn has_row_win(currently_called: &[u32], board: &Board, index_to_check: usize) -> bool {
    (0_usize..=4_usize).all(|j| currently_called.contains(&board.0[index_to_check][j]))
}

fn has_column_win(currently_called: &[u32], board: &Board, index_to_check: usize) -> bool {
    (0_usize..=4_usize).all(|j| currently_called.contains(&board.0[j][index_to_check]))
}

fn calculate_unmarked_sum(board: &Board, called_numbers: &[u32]) -> u32 {
    let mut sum = 0;
    for i in 0..=4 {
        for j in 0..=4 {
            if !called_numbers.contains(&board.0[i][j]) {
                sum += board.0[i][j];
            }
        }
    }
    sum
}

#[test]
fn test_part1() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    let boards = read_boards(&sample_data);
    println!("boards: {:?}", boards);
    assert_eq!(3, boards.len());
    assert_eq!(4512, part1(&sample_data));
}

#[test]
fn test_part2() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(1924, part2(&sample_data));
}
