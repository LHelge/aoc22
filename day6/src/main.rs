use std::fs;

fn contains_doubles(signal: &str) -> bool {
    let mut s = String::from(signal);
    
    while s.len() > 0 {
        let c = s.pop().unwrap();

        if s.contains(c) {
            return true;
        }
    }

    // No double found
    false
}

fn find_marker(signal: &str, len: usize) -> Option<usize> {
    for n in len..signal.len() {
        if !contains_doubles(&signal[n-len..n]) {
            return Some(n);
        }
    }
    None
}

fn main() {
    // Read file
    let filename = "input.txt";
    let input = match fs::read_to_string(filename) {
        Ok(f) => f,
        Err(e) => panic!("Problem reading file {}: {}", filename, e)
    };

    println!("Result1: {}", find_marker(&input, 4).unwrap_or(0));
    println!("Result2: {}", find_marker(&input, 14).unwrap_or(0));
}