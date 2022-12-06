# Advent of Code 2022
This is my repo for solving (Advent of Code 2022)[https://adventofcode.com/2022] challenges while learning Rust at the same time.

## Boilerplate
Here are some boilerplate code to get started on each day:
```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read file
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    // Do stuff
    let mut result = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        // Do stuff per line
    }

    // print result
    println!("Result: {}", result)
}
```