use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read file
    //let filename = "input.txt";
    let filename = "input_example.txt";
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => panic!("Problem reading file {}: {}", filename, e)
    };
    let reader = BufReader::new(file);

    // Do stuff
    let mut result1 = 0;
    let mut result2 = 0;
    for (i, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => panic!("Bad input on line {}: {}", i, e)
        };

        println!("{}", line);

        // Solve Part 1
        result1 += 1;

        // Solve Part 2
        result2 += 1;
    }

    // print result
    println!("Answer part one: {}", result1);
    println!("Answer part two: {}", result2);
}
