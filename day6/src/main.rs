use std::fs;

fn main() {
    // Read file
    //let filename = "input.txt";
    let filename = "input_example.txt";
    let input = match fs::read_to_string(filename) {
        Ok(f) => f,
        Err(e) => panic!("Problem reading file {}: {}", filename, e)
    };

    // Do stuff
    println!("{}", input);
}