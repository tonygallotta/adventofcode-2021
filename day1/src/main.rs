use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading file {}", filename);
    let file = fs::File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let mut all_values: Vec<u32> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        all_values.push(line.unwrap().parse().unwrap());
    }
    println!("Part 1: {}", count_windowed_increases(1, &all_values));
    println!("Part 2: {}", count_windowed_increases(3, &all_values))
}

fn count_windowed_increases(window_size: usize, values: &Vec<u32>) -> u32 {
    let mut count = 0;
    for i in 0..values.len() - window_size {
        if values[i + window_size] > values[i] {
            count = count + 1;
        }
    }
    count
}

#[test]
fn test() {
    let sample_values = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(7, count_windowed_increases(1, &sample_values));
    assert_eq!(5, count_windowed_increases(3, &sample_values));
}
