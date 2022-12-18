use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

#[allow(dead_code)]
struct Entry {
    compartment_1: String,
    compartment_2: String,
    raw_entry: String,
    compartment_1_map: HashMap<char, u32>,
    compartment_2_map: HashMap<char, u32>,
    sac_map: HashMap<char, u32>,
    common: Vec<char>,
}

fn parse_input(file: String) -> Vec<Entry> {
    let mut input_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_file.push(file);

    let mut entries: Vec<Entry> = Vec::new();
    let file = match File::open(input_file) {
        Ok(res) => res,
        Err(_) => panic!("Could Not Read File"),
    };

    let reader = BufReader::new(file);

    for line_res in reader.lines() {
        let line: String = match line_res {
            Ok(line) => line,
            Err(_) => panic!("Problem Reading Line!"),
        };

        let mid_point = line.len() / 2;
        let compartment_1 = &line[..mid_point];
        let compartment_2 = &line[mid_point..];

        let mut compartment_1_map: HashMap<char, u32> = HashMap::new();
        let mut compartment_2_map: HashMap<char, u32> = HashMap::new();
        let mut sac_map: HashMap<char, u32> = HashMap::new();
        let mut common: Vec<char> = Vec::new();

        for c in compartment_1.chars() {
            *compartment_1_map.entry(c).or_insert(0) += 1;
            *sac_map.entry(c).or_insert(0) += 1;
        }

        for c in compartment_2.chars() {
            if compartment_1_map.contains_key(&c) && !compartment_2_map.contains_key(&c) {
                common.push(c);
            }
            *compartment_2_map.entry(c).or_insert(0) += 1;
            *sac_map.entry(c).or_insert(0) += 1;
        }

        entries.push(Entry {
            compartment_1: String::from(compartment_1),
            compartment_2: String::from(compartment_2),
            raw_entry: line,
            compartment_1_map,
            compartment_2_map,
            sac_map,
            common,
        });
    }

    return entries;
}

fn get_sum_part_1() -> u32 {
    let mut sum: u32 = 0;

    let entries: Vec<Entry> = parse_input(String::from("input_1"));
    for entry in entries.iter() {
        for c in entry.common.iter() {
            sum += get_priority(*c);
        }
    }

    return sum;
}

fn get_sum_part_2() -> u32 {
    let mut sum: u32 = 0;

    let entries: Vec<Entry> = parse_input(String::from("input_2"));

    for i in (0..entries.len()).step_by(3) {
        let one_two_common: Vec<char> = entries[i]
            .sac_map
            .keys()
            .filter(|key| entries[i + 1].sac_map.contains_key(key))
            .cloned()
            .collect();

        let common: Vec<char> = one_two_common
            .iter()
            .filter(|key| entries[i + 2].sac_map.contains_key(key))
            .cloned()
            .collect();

        for k in common.iter() {
            sum += get_priority(*k);
        }
    }

    return sum;
}

fn get_priority(item: char) -> u32 {
    if item.is_uppercase() {
        return item as u32 - 38;
    } else if item.is_lowercase() {
        return item as u32 - 96;
    } else {
        panic!("Could Not Parse Char to Priority");
    }
}

fn main() {
    println!("Part 1 Sum: {}", get_sum_part_1());
    println!("Part 2 Sum: {}", get_sum_part_2());
}
