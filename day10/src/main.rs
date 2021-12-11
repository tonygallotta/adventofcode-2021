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
    lines.iter().map(|l| syntax_error_score(l)).sum()
}

fn part2(lines: &Vec<String>) -> u64 {
    let mut autocomplete_scores = Vec::new();
    for line in lines {
        if syntax_error_score(line) == 0 {
            autocomplete_scores.push(score_autocomplete(&autocomplete(line)));
        }
    }
    autocomplete_scores.sort();
    autocomplete_scores[(autocomplete_scores.len() - 1) / 2]
}

fn autocomplete(line: &String) -> String {
    let mut result = String::new();
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        if is_opener(c) {
            stack.push(c);
        } else {
            stack.pop();
        }
    }
    for opener in stack.iter().rev() {
        result.push(closer_for(opener.clone()));
    }
    result
}

fn syntax_error_score(line: &String) -> u64 {
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        if is_opener(c) {
            stack.push(c);
        } else {
            let opener = stack.pop();
            if opener.is_none() || !is_pair(opener.unwrap(), c) {
                return score_syntax_error(c);
            }
        }
    }
    0
}

fn is_opener(c: char) -> bool {
    c == '(' || c == '[' || c == '{' || c == '<'
}

fn is_pair(opener: char, closer: char) -> bool {
    match opener {
        '(' => closer == ')',
        '[' => closer == ']',
        '{' => closer == '}',
        '<' => closer == '>',
        _ => false,
    }
}

fn closer_for(opener: char) -> char {
    match opener {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => 'x',
    }
}

fn score_syntax_error(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn score_autocomplete(added: &String) -> u64 {
    let mut score = 0;
    for c in added.chars() {
        score = score * 5
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
            }
    }
    score
}

#[test]
fn test_part1() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(26397, part1(&sample_data));
}

#[test]
fn test_part2() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(
        "}}]])})]",
        autocomplete(&String::from("[({(<(())[]>[[{[]{<()<>>"))
    );
    assert_eq!(
        0,
        syntax_error_score(&String::from("[({(<(())[]>[[{[]{<()<>>"))
    );
    assert_eq!(
        288957,
        score_autocomplete(&autocomplete(&String::from("[({(<(())[]>[[{[]{<()<>>")))
    );
    assert_eq!(288957, part2(&sample_data));
}
