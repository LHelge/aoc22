use std::fs::File;
use std::io::{BufRead, BufReader};

fn priority(item: char) -> Option<u32> {
    match item {
        'a' ..= 'z' => Some(item as u32 - 'a' as u32 + 1),
        'A' ..= 'Z' => Some(item as u32 - 'A' as u32 + 27),
        _ => None
    }
}


fn main() {
    // Read file
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    // Do stuff
    let mut result1: u32 = 0;
    let mut result2: u32 = 0;

    let mut group: Vec<&str> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let items = line.as_str();

        // Part 1
        let count = items.len();
        let first_compartment = &items[..count/2];
        let second_compartment = &items[count/2..];
        for c in first_compartment.chars() {
            if second_compartment.contains(c) {
                result1 += priority(c).unwrap_or(0);
                break;
            }
        }

        // Part 2
        group.push(items.clone());
        if group.len() >= 3 {
            for c in group[0].chars() {
                if group[1].contains(c) && group[2].contains(c) {
                    result2 += priority(c).unwrap_or(0);
                }
            }
            group.clear();
        }
    }

    // print result
    println!("Answer part one: {}", result1);
    println!("Answer part two: {}", result2);
}