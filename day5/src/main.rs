use std::fs::File;
use std::io::{BufRead, BufReader};

// Helper function to initialize the stack of crates from the input file
// Much quicker to do like this than to parse it.
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

// Get the top crate of each stack as a string
fn top_of_stack(stacks: &Vec<String>) -> String {
    let mut top = String::new();

    for stack in stacks {
        top.push(stack.chars().last().unwrap());
    }

    top
}

fn main() {
    // Read file
    let filename = "input.txt";
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => panic!("Problem reading file {}: {}", filename, e)
    };
    let reader = BufReader::new(file);

    // Initialize two instances of the stacks, one for each part of the puzzle
    let mut stacks1 = init_stacks();
    let mut stacks2 = init_stacks();

    // Go through the input file, line by line
    for (i, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => panic!("Bad input on line {}: {}", i, e)
        };

        // Parse the unput
        let step: Vec<&str> = line.split(' ').collect();
        let amount: usize = step[1].parse().unwrap();
        let from: usize = step[3].parse().unwrap();
        let to: usize = step[5].parse().unwrap();
        
        // Part 1
        for _n in 0..amount {
            // Move one crate from the old stack to the new
            let tmp1 = stacks1[from-1].pop().unwrap();
            stacks1[to-1].push(tmp1);
        }

        // Part 2
        let mut tmp2 = String::new();
        // Put ammount of crates in temporary stack
        for _n in 0..amount {
            tmp2.push(stacks2[from-1].pop().unwrap());
        }
        // Put them back in the new stack
        for _n in 0..tmp2.len() {
            stacks2[to-1].push(tmp2.pop().unwrap());
        }
    }

    // print result
    println!("Result 1: {}", top_of_stack(&stacks1));
    println!("Result 2: {}", top_of_stack(&stacks2));
}