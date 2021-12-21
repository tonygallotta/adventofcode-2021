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
    let part_2_answer = part2(&parsed_lines, 50, |i| i % 2 == 0);
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
    let image_enhancement_algo = &lines[0].replace('.', "0").replace('#', "1");
    let mut input_image: Vec<String> = Vec::new();
    for line in &lines[2..] {
        input_image.push(line.clone());
    }
    let output_image = enhance(image_enhancement_algo, &input_image, false);
    // println!("Input");
    // for line in &input_image {
    //     println!("{}", line);
    // }
    // println!("Output");
    // for line in &output_image {
    //     println!("{}", line);
    // }
    // println!("Output (2)");
    let twice_ehanced = &enhance(image_enhancement_algo, &output_image, true);
    // for line in twice_ehanced {
    //     println!("{}", line);
    // }
    twice_ehanced
        .iter()
        .map(|l| l.chars().filter(|c| *c == '#').count())
        .sum()
}

fn enhance(
    image_enhancement_algo: &String,
    input_image: &Vec<String>,
    assume_infinite_lit: bool,
) -> Vec<String> {
    let mut output_image: Vec<String> = Vec::new();
    for i in -1..=input_image.len() as isize {
        let mut current_row = String::new();
        for j in -1..=input_image[0].len() as isize {
            if is_lit_after_enhance(
                (i, j),
                image_enhancement_algo,
                &input_image,
                assume_infinite_lit,
            ) {
                current_row.push('#');
            } else {
                current_row.push('.');
            }
        }
        output_image.push(current_row);
    }
    output_image
}

fn is_lit_after_enhance(
    coords: (isize, isize),
    algo: &String,
    input_image: &Vec<String>,
    assume_infinite_lit: bool,
) -> bool {
    let total_rows = input_image.len();
    let total_cols = input_image[0].len();
    let (i, j) = coords;
    let mut output_binary = if assume_infinite_lit {
        "111111111".to_string()
    } else {
        "000000000".to_string()
    };
    let mut binary_index = 0;
    for x in vec![i - 1, i, i + 1] {
        for y in vec![j - 1, j, j + 1] {
            if x < 0 || y < 0 {
                binary_index += 1;
                continue;
            }
            let x_usize = x as usize;
            let y_usize = y as usize;
            if x_usize < total_rows && y_usize < total_cols {
                if input_image[x_usize].chars().nth(y_usize).unwrap() == '#' {
                    output_binary.replace_range(binary_index..=binary_index, "1");
                } else {
                    output_binary.replace_range(binary_index..=binary_index, "0");
                }
            }
            binary_index += 1;
        }
    }
    algo.chars()
        .nth(usize::from_str_radix(&output_binary, 2).unwrap())
        .unwrap()
        == '1'
}

fn part2(
    lines: &Vec<String>,
    num_enhancements: u32,
    assume_infinite_lit: fn(u32) -> bool,
) -> usize {
    let image_enhancement_algo = &lines[0].replace('.', "0").replace('#', "1");
    let mut input_image: Vec<String> = Vec::new();
    for line in &lines[2..] {
        input_image.push(line.clone());
    }
    let mut output_image = input_image.clone();
    for i in 1..=num_enhancements {
        output_image = enhance(
            image_enhancement_algo,
            &output_image,
            assume_infinite_lit(i),
        );
    }
    output_image
        .iter()
        .map(|l| l.chars().filter(|c| *c == '#').count())
        .sum()
}

#[test]
fn test_part1() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(35, part1(&sample_data));
}

#[test]
fn test_part2() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(3351, part2(&sample_data, 50, |_| false));
}
