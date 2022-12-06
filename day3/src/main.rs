use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read file
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    // Do stuff
    let mut result1 = 0;
    let mut result2 = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        // Do stuff per line
    }

    // print result
    println!("Answer part one: {}", result1);
    println!("Answer part two: {}", result2);
}