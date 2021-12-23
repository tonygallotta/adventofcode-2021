use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::time::Instant;

struct Packet {
    version: u64,
    type_id: u64,
    literal_value: Option<u64>,
    contained_packets: Vec<Packet>,
}

impl Packet {
    fn version_sum(&self) -> u64 {
        self.version
            + self
                .contained_packets
                .iter()
                .map(|p| p.version_sum())
                .sum::<u64>()
    }

    fn eval(&self) -> u64 {
        self.literal_value.unwrap_or_else(|| match self.type_id {
            0 => self.contained_packets.iter().map(|p| p.eval()).sum(),
            1 => self
                .contained_packets
                .iter()
                .map(|p| p.eval())
                .reduce(|acc, val| acc * val)
                .unwrap_or(0),
            2 => self
                .contained_packets
                .iter()
                .map(|p| p.eval())
                .reduce(|acc, val| if val < acc { val } else { acc })
                .unwrap_or(0),
            3 => self
                .contained_packets
                .iter()
                .map(|p| p.eval())
                .reduce(|acc, val| if val > acc { val } else { acc })
                .unwrap_or(0),
            5 => {
                if self.contained_packets.first().unwrap().eval()
                    > self.contained_packets.last().unwrap().eval()
                {
                    1
                } else {
                    0
                }
            }
            6 => {
                if self.contained_packets.first().unwrap().eval()
                    < self.contained_packets.last().unwrap().eval()
                {
                    1
                } else {
                    0
                }
            }
            7 => {
                if self.contained_packets.first().unwrap().eval()
                    == self.contained_packets.last().unwrap().eval()
                {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        })
    }
}

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
    let packets = read_packets(&to_binary(lines.first().unwrap()));
    packets.iter().map(|p| p.version_sum()).sum()
}

fn part2(lines: &Vec<String>) -> u64 {
    let packets = read_packets(&to_binary(lines.first().unwrap()));
    packets.iter().map(|p| p.eval()).sum()
}

fn read_packets(outermost_packet: &String) -> Vec<Packet> {
    let mut current_offset = 0;
    let mut packets = Vec::new();
    while current_offset < outermost_packet.len() {
        let (packet, next_offset) = read_packet(current_offset, &outermost_packet);
        packets.push(packet);
        current_offset = next_offset;
        if outermost_packet
            .chars()
            .skip(current_offset)
            .all(|c| c == '0')
        {
            break;
        }
    }
    packets
}

fn read_packet(start_offset: usize, outermost_packet: &String) -> (Packet, usize) {
    let mut current_offset = start_offset;
    let header = outermost_packet
        .chars()
        .skip(current_offset)
        .take(6)
        .collect::<String>();
    let version =
        u64::from_str_radix(header.chars().take(3).collect::<String>().as_str(), 2).unwrap();
    let type_id = u64::from_str_radix(
        header.chars().skip(3).take(3).collect::<String>().as_str(),
        2,
    )
    .unwrap();
    current_offset += header.len();
    match type_id {
        4 => {
            let mut literal_value = String::new();
            loop {
                let next_group: Vec<char> = outermost_packet
                    .chars()
                    .skip(current_offset)
                    .take(5)
                    .collect();
                literal_value.push_str(next_group.iter().skip(1).collect::<String>().as_str());
                current_offset += 5;
                if next_group[0] == '1' {
                    continue;
                } else {
                    break;
                }
            }
            return (
                Packet {
                    version,
                    type_id,
                    literal_value: Some(u64::from_str_radix(&*literal_value, 2).unwrap()),
                    contained_packets: vec![],
                },
                current_offset,
            );
        }
        _ => {
            let length_type_id = outermost_packet
                .chars()
                .skip(current_offset)
                .nth(0)
                .unwrap();
            current_offset += 1;
            let mut contained_packets = Vec::new();
            if length_type_id == '0' {
                let remaining_length = u32::from_str_radix(
                    &*outermost_packet
                        .chars()
                        .skip(current_offset)
                        .take(15)
                        .collect::<String>(),
                    2,
                )
                .unwrap();
                current_offset += 15;
                let end_offset = current_offset + remaining_length as usize;
                while current_offset < end_offset {
                    let (packet, next_packet_start) = read_packet(current_offset, outermost_packet);
                    contained_packets.push(packet);
                    current_offset = next_packet_start;
                }
            } else {
                let num_contained_packets = u32::from_str_radix(
                    &*outermost_packet
                        .chars()
                        .skip(current_offset)
                        .take(11)
                        .collect::<String>(),
                    2,
                )
                .unwrap();
                current_offset += 11;
                for _ in 0..num_contained_packets {
                    let (packet, next_packet_start) = read_packet(current_offset, outermost_packet);
                    contained_packets.push(packet);
                    current_offset = next_packet_start;
                }
            }
            return (
                Packet {
                    version,
                    type_id,
                    literal_value: None,
                    contained_packets,
                },
                current_offset,
            );
        }
    }
}

fn to_binary(hex: &String) -> String {
    hex.chars()
        .map(|c| format!("{:04b}", u8::from_str_radix(&*c.to_string(), 16).unwrap()))
        .collect()
}

#[test]
fn test_part1() {
    assert_eq!(16, part1(&vec!["8A004A801A8002F478".to_string()]));
    assert_eq!(12, part1(&vec!["620080001611562C8802118E34".to_string()]));
    assert_eq!(23, part1(&vec!["C0015000016115A2E0802F182340".to_string()]));
    assert_eq!(
        31,
        part1(&vec!["A0016C880162017C3686B18A3D4780".to_string()])
    );
}

#[test]
fn test_part2() {
    assert_eq!(3, part2(&vec!["C200B40A82".to_string()]));
    assert_eq!(54, part2(&vec!["04005AC33890".to_string()]));
    assert_eq!(7, part2(&vec!["880086C3E88112".to_string()]));
    assert_eq!(9, part2(&vec!["CE00C43D881120".to_string()]));
    assert_eq!(1, part2(&vec!["D8005AC2A8F0".to_string()]));
    assert_eq!(0, part2(&vec!["F600BC2D8F".to_string()]));
    assert_eq!(0, part2(&vec!["9C005AC2F8F0".to_string()]));
    assert_eq!(1, part2(&vec!["9C0141080250320F1802104A08".to_string()]));
}
