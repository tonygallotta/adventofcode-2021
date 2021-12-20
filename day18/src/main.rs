use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::num::ParseIntError;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let filename = env::args().nth(1).unwrap_or(String::from("input.txt"));
    let parsed_lines = read_file_to_vec(filename);
    let part_1_answer = part1(&parsed_lines);
    println!(
        "Part 1 ({}ms): {}",
        now.elapsed().as_millis(),
        part_1_answer,
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
    let mut result = reduce(&lines[0]);
    for i in 1..lines.len() {
        let reduced_sum = reduce(&add(&result, &lines[i]));
        // println!("{}\n+{} = {}", &result, &lines[i], &reduced_sum);
        result = reduced_sum;
    }
    magnitude(&result)
}

fn part2(lines: &Vec<String>) -> u64 {
    let mut max = 0;
    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i == j {
                continue;
            }
            let reduced_sum = &reduce(&add(&lines[i], &lines[j]));
            // println!("Checking {}", reduced_sum);
            let magnitude_of_sum = magnitude(reduced_sum);
            if magnitude_of_sum > max {
                max = magnitude_of_sum;
            }
        }
    }
    max
}

fn add(first: &String, second: &String) -> String {
    format!("[{},{}]", first, second)
}

fn reduce(snailfish_number: &String) -> String {
    let mut new_snailfish_number = snailfish_number.clone();
    loop {
        let exploded = explode(&new_snailfish_number);
        if exploded == new_snailfish_number {
            let split = split(&new_snailfish_number);
            if split == new_snailfish_number {
                return new_snailfish_number;
            }
            // println!("after split: {}", split);
            new_snailfish_number = split;
        } else {
            // println!("after explode: {}", exploded);
            new_snailfish_number = exploded;
        }
    }
}

fn explode(snailfish_number: &String) -> String {
    let mut new_snailfish_number = String::new();
    let mut nesting_level = 0;
    let mut i = 0;
    let mut explosion_done = false;
    while i < snailfish_number.len() {
        let current_char = snailfish_number.chars().nth(i).unwrap();
        if current_char == '[' {
            nesting_level += 1;
        } else if current_char == ']' {
            nesting_level -= 1;
        }
        if nesting_level >= 5 && !explosion_done {
            let exploding_left: usize = snailfish_number[i + 1..]
                .chars()
                .take_while(|c| c.is_digit(10))
                .collect::<String>()
                .parse()
                .unwrap();
            let exploding_right_start_idx = (i + exploding_left.to_string().len()) + 2;
            let exploding_right: usize = snailfish_number[exploding_right_start_idx..]
                .chars()
                .take_while(|c| c.is_digit(10))
                .collect::<String>()
                .parse()
                .unwrap();
            let number_to_left: Result<usize, ParseIntError> = snailfish_number[0..i]
                .chars()
                .rev()
                .skip_while(|c| !c.is_digit(10))
                .take_while(|c| c.is_digit(10))
                .collect::<String>()
                .chars()
                .rev()
                .collect::<String>()
                .parse();
            if number_to_left.is_ok() {
                let unwrapped_left = number_to_left.unwrap();
                let num_to_back_up = snailfish_number[0..i]
                    .chars()
                    .rev()
                    .take_while(|c| !c.is_digit(10))
                    .count()
                    + unwrapped_left.to_string().len();
                let mut to_add_back = Vec::new();
                for _ in 0..num_to_back_up {
                    let popped = new_snailfish_number.pop();
                    if !popped.unwrap().is_digit(10) {
                        to_add_back.push(popped.unwrap());
                    }
                }
                new_snailfish_number.push_str(&(unwrapped_left + exploding_left).to_string());
                to_add_back.reverse();
                for c in to_add_back {
                    new_snailfish_number.push(c);
                }
            }
            new_snailfish_number.push('0');
            let exploding_right_end_idx =
                exploding_right_start_idx + exploding_right.to_string().len();
            let number_to_right: Result<usize, ParseIntError> = snailfish_number
                [exploding_right_end_idx + 1..]
                .chars()
                .skip_while(|c| !c.is_digit(10))
                .take_while(|c| c.is_digit(10))
                .collect::<String>()
                .parse();
            if number_to_right.is_ok() {
                let non_digits_to_append = &snailfish_number[exploding_right_end_idx + 1..]
                    .chars()
                    .take_while(|c| !c.is_digit(10))
                    .collect::<String>();
                new_snailfish_number.push_str(non_digits_to_append);
                let parsed_number_to_right = number_to_right.unwrap();
                let updated_right = &(parsed_number_to_right + exploding_right).to_string();
                new_snailfish_number.push_str(updated_right);
                i = exploding_right_end_idx
                    + non_digits_to_append.len()
                    + parsed_number_to_right.clone().to_string().len()
                    + 1;
            } else {
                i = exploding_right_end_idx + 1;
            }
            explosion_done = true;
            nesting_level -= 1;
        } else {
            new_snailfish_number.push(current_char);
            i += 1;
        }
    }
    new_snailfish_number
}

fn split(snailfish_number: &String) -> String {
    let mut new_snailfish_number = String::new();
    let mut i = 0;
    let mut split_done = false;
    while i < snailfish_number.len() {
        let current_char = snailfish_number.chars().nth(i).unwrap();
        if !split_done && current_char.is_digit(10) {
            let current_number: usize = snailfish_number[i..]
                .chars()
                .take_while(|c| c.is_digit(10))
                .collect::<String>()
                .parse()
                .unwrap();
            if current_number >= 10 {
                new_snailfish_number.push('[');
                new_snailfish_number
                    .push_str(&((current_number as f64) / 2_f64).floor().to_string());
                new_snailfish_number.push(',');
                new_snailfish_number
                    .push_str(&((current_number as f64) / 2_f64).ceil().to_string());
                new_snailfish_number.push(']');
                split_done = true;
                i += current_number.to_string().len();
            } else {
                new_snailfish_number.push(current_char);
                i += 1;
            }
        } else {
            new_snailfish_number.push(current_char);
            i += 1;
        }
    }
    new_snailfish_number
}

fn magnitude(snailfish_number: &String) -> u64 {
    // println!("Checking magnitude of {}", snailfish_number);
    let pair = split_to_pair(&snailfish_number);
    magnitude_recurse(pair.0) * 3 + magnitude_recurse(pair.1) * 2
}

fn magnitude_recurse(snailfish_number: &str) -> u64 {
    if snailfish_number.len() == 1 {
        return snailfish_number.parse().unwrap();
    }
    let pair = split_to_pair(snailfish_number);
    magnitude_recurse(pair.0) * 3 + magnitude_recurse(pair.1) * 2
}

fn split_to_pair(snailfish_number: &str) -> (&str, &str) {
    let nesting_level = snailfish_number.chars().take_while(|c| *c == '[').count();
    if nesting_level == 0 {
        return snailfish_number.split_once(',').unwrap();
    }
    let mut pairs_closed: i32 = 0;
    let mut split_position = nesting_level;
    while pairs_closed < (nesting_level - 1) as i32 {
        let current_char = snailfish_number.chars().nth(split_position).unwrap();
        if current_char == ']' {
            pairs_closed += 1;
        } else if current_char == '[' {
            pairs_closed -= 1;
        }
        split_position += 1;
    }
    if snailfish_number.chars().nth(split_position).unwrap() != ',' {
        split_position += 1;
    }
    (
        &snailfish_number[1..split_position],
        &snailfish_number[split_position + 1..snailfish_number.len() - 1],
    )
}

#[test]
fn test_part1() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(4140, part1(&sample_data));
}

#[test]
fn test_explode() {
    println!("{}", explode(&String::from("[[[[[9,8],1],2],3],4]")));
    assert_eq!(
        String::from("[[[[0,9],2],3],4]"),
        explode(&String::from("[[[[[9,8],1],2],3],4]"))
    );
    assert_eq!(
        String::from("[7,[6,[5,[7,0]]]]"),
        explode(&String::from("[7,[6,[5,[4,[3,2]]]]]"))
    );
    assert_eq!(
        String::from("[[6,[5,[7,0]]],3]"),
        explode(&String::from("[[6,[5,[4,[3,2]]]],1]"))
    );
    assert_eq!(
        String::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
        explode(&String::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"))
    );
    assert_eq!(
        String::from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"),
        explode(&String::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"))
    );
    assert_eq!(
        String::from("[[[[0,7],4],[15,[0,13]]],[1,1]]"),
        explode(&String::from("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"))
    );
}

#[test]
fn test_split() {
    assert_eq!(
        String::from("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"),
        split(&String::from("[[[[0,7],4],[15,[0,13]]],[1,1]]"))
    );
}

#[test]
fn test_reduce() {
    // assert_eq!(
    //     "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_string(),
    //     reduce(&"[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".to_string())
    // );
    assert_eq!(
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]".to_string(),
        reduce(
            &"[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]"
                .to_string()
        )
    );
}

#[test]
fn test_reduced_add() {
    assert_eq!(
        "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]".to_string(),
        reduce(&&"[[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]],[2,9]]".to_string())
    );
}

#[test]
fn test_magnitude() {
    assert_eq!(
        3675,
        magnitude(&&"[[[[7,0],[8,7]],[[7,7],[8,8]]],[[[5,6],[6,6]],[[5,6],[6,0]]]]".to_string())
    );
    assert_eq!(143, magnitude(&&"[[1,2],[[3,4],5]]".to_string()));
    assert_eq!(
        3993,
        magnitude(&&"[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]".to_string())
    );
}

#[test]
fn test_part2() {
    let sample_data = read_file_to_vec(String::from("test_2.txt"));
    assert_eq!(3993, part2(&sample_data));
}
