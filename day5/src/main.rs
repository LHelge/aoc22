use std::fs::File;
use std::io::{BufRead, BufReader};

fn init_stacks() -> Vec<String> {
    let mut stacks: Vec<String> = Vec::new();

    // Real
    stacks.push(String::from("GFVHPS"));
    stacks.push(String::from("GJFBVDZM"));
    stacks.push(String::from("GMLJN"));
    stacks.push(String::from("NGZVDWP"));
    stacks.push(String::from("VRCB"));
    stacks.push(String::from("VRSMPWLZ"));
    stacks.push(String::from("THP"));
    stacks.push(String::from("QRSNCHZV"));
    stacks.push(String::from("FLGPVQJ"));

    stacks
}

fn main() {
    // Read file
    let filename = "input.txt";
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => panic!("Problem reading file {}: {}", filename, e)
    };
    let reader = BufReader::new(file);

    // Initialize stacks
    let mut stacks1 = init_stacks();
    let mut stacks2 = init_stacks();

    // Do stuff
    for (i, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => panic!("Bad input on line {}: {}", i, e)
        };

        let step: Vec<&str> = line.split(' ').collect();
        let ammount: usize = step[1].parse().unwrap();
        let from: usize = step[3].parse().unwrap();
        let to: usize = step[5].parse().unwrap();
        
        let mut tmp2 = String::new();
        for _n in 0..ammount {
            // Part 1
            let tmp1 = stacks1[from-1].pop().unwrap();
            stacks1[to-1].push(tmp1);

            // Part 2
            tmp2.push(stacks2[from-1].pop().unwrap());
        }
        for _n in 0..tmp2.len() {
            stacks2[to-1].push(tmp2.pop().unwrap());
        }
    }

    // print result
    let mut result1 = String::new();
    for mut stack in stacks1 {
        result1.push(stack.pop().unwrap());
    }

    let mut result2 = String::new();
    for mut stack in stacks2 {
        result2.push(stack.pop().unwrap());
    }
    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
}