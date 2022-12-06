use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Sections {
    from: usize,
    to: usize
}

impl Sections {
    fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.split('-').collect();
        if parts.len() == 2 {
            return Self {
                from: parts[0].parse().unwrap_or(0),
                to: parts[1].parse().unwrap_or(0)
            }
        }
        Self { from: 0, to: 0 }
    }

    fn contains(&self, other: &Sections) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn overlap(&self, other: &Sections) -> bool {
        (self.from >= other.from && self.from <= other.to) || 
        (self.to   >= other.from && self.to   <= other.to) ||
        self.contains(other)
    }
}


fn main() {
    // Read file
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    // Do stuff
    let mut result1 = 0;
    let mut result2 = 0;
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let pairs: Vec<&str> = line.split(',').collect();
        if pairs.len() != 2 {
            panic!("Bad input on line {}: {}", i, line);
        }

        let first = Sections::new(pairs[0]);
        let second = Sections::new(pairs[1]);

        if first.contains(&second) || second.contains(&first) {
            result1 += 1;
        }

        if first.overlap(&second) {
            result2 += 1;
        }


        // Do stuff per line
    }

    // print result
    println!("Answer part one: {}", result1);
    println!("Answer part two: {}", result2);
}