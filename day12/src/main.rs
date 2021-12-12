use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::iter::FromIterator;
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
    let (vertices, vertex_to_index, edges) = build_graph(lines);
    spelunk(
        *vertex_to_index.get("start").unwrap(),
        *vertex_to_index.get("end").unwrap(),
        &vertices,
        &edges,
        false,
    )
}

fn build_graph(lines: &Vec<String>) -> (Vec<&str>, HashMap<&str, usize>, Vec<Vec<bool>>) {
    let mut vertices: Vec<&str> = lines
        .iter()
        .flat_map(|line| line.split("-"))
        .into_iter()
        .collect();
    vertices.sort();
    vertices.dedup();
    let vertex_to_index: HashMap<&str, usize> =
        HashMap::from_iter(vertices.iter().enumerate().map(|(i, v)| (*v, i)));
    let mut edges = vec![vec![false; vertices.len()]; vertices.len()];
    for line in lines {
        let mut parts = line.split("-");
        let vertex_1 = parts.next().unwrap();
        let vertex_2 = parts.next().unwrap();
        let vertex_1_index = *vertex_to_index.get(vertex_1).unwrap();
        let vertex_2_index = *vertex_to_index.get(vertex_2).unwrap();
        edges[vertex_1_index][vertex_2_index] = true;
        edges[vertex_2_index][vertex_1_index] = true;
    }
    (vertices, vertex_to_index, edges)
}

fn spelunk(
    start_index: usize,
    end_index: usize,
    vertices: &Vec<&str>,
    edges: &Vec<Vec<bool>>,
    double_visit_allowed: bool,
) -> usize {
    let mut visited_small_caves: HashSet<usize> = HashSet::new();
    let mut current_path: Vec<usize> = Vec::new();
    let mut all_paths: Vec<Vec<usize>> = Vec::new();
    let mut double_visited_vertex = Cell::from(Option::None);
    dfs(
        start_index,
        end_index,
        vertices,
        edges,
        &mut visited_small_caves,
        &mut current_path,
        &mut all_paths,
        double_visit_allowed,
        &mut double_visited_vertex,
    );

    // for path in all_paths.clone() {
    //     for (idx, node) in path.iter().enumerate() {
    //         print!("{}", vertices[*node]);
    //         if idx != path.len() - 1 {
    //             print!("->");
    //         }
    //     }
    //     println!();
    // }
    all_paths.len()
}

fn dfs(
    start_index: usize,
    end_index: usize,
    vertices: &Vec<&str>,
    edges: &Vec<Vec<bool>>,
    visited_small_caves: &mut HashSet<usize>,
    current_path: &mut Vec<usize>,
    all_paths: &mut Vec<Vec<usize>>,
    double_visit_allowed: bool,
    double_visited_vertex: &mut Cell<Option<usize>>,
) {
    let vertex_name = vertices[start_index];
    if visited_small_caves.contains(&start_index) {
        if vertex_name != "end"
            && vertex_name != "start"
            && double_visit_allowed
            && double_visited_vertex.get().is_none()
        {
            double_visited_vertex.set(Option::Some(start_index));
        } else {
            return;
        }
    }
    if vertex_name.chars().all(|c| c.is_lowercase()) {
        visited_small_caves.insert(start_index);
    }
    current_path.push(start_index);
    if start_index == end_index {
        all_paths.push(current_path.clone());
        current_path.pop();
        visited_small_caves.remove(&start_index);
        return;
    }
    for adjacent in edges[start_index]
        .iter()
        .enumerate()
        .filter(|(_, &exists)| exists)
        .map(|(idx, _)| idx)
    {
        dfs(
            adjacent,
            end_index,
            vertices,
            edges,
            visited_small_caves,
            current_path,
            all_paths,
            double_visit_allowed,
            double_visited_vertex,
        );
    }
    current_path.pop();
    if double_visited_vertex.get().is_some() && double_visited_vertex.get().unwrap() == start_index
    {
        double_visited_vertex.set(Option::None);
    } else {
        visited_small_caves.remove(&start_index);
    }
}

fn part2(lines: &Vec<String>) -> usize {
    let (vertices, vertex_to_index, edges) = build_graph(lines);
    spelunk(
        *vertex_to_index.get("start").unwrap(),
        *vertex_to_index.get("end").unwrap(),
        &vertices,
        &edges,
        true,
    )
}

#[test]
fn test_part1() {
    let sample_data = read_file_to_vec(String::from("test.txt"));
    assert_eq!(19, part1(&sample_data));
}

#[test]
fn test_part2() {
    let sample_small = read_file_to_vec(String::from("test_small.txt"));
    assert_eq!(36, part2(&sample_small));
    let sample_larger = read_file_to_vec(String::from("test.txt"));
    assert_eq!(103, part2(&sample_larger));
}
