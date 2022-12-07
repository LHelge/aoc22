use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Command {
    command: String,
    arguments: Vec<String>,
    outputs: Vec<String>
}

impl Command {
    fn new(input: &Vec<String>) -> Self {
        let parts: Vec<&str> = input[0].split(' ').collect();

        let mut cmd = Self { 
            command: String::from(parts[1]),
            arguments: Vec::new(), 
            outputs: Vec::new() 
        };

        for n in 2..parts.len() {
            cmd.arguments.push(String::from(parts[n]));
        }
        for n in 1..input.len() {
            cmd.outputs.push(input[n].clone());
        }

        cmd
    }
}


fn parse_command(cmd: &Command) {

    match cmd.command.as_str() {
        "cd" => println!("Change directory to {}", cmd.arguments[0]),
        "ls" => println!("Listed directory contents {:?}", cmd.outputs),
        _ => panic!("Unsupported command: {}", cmd.command)
    }

}

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

    let mut command: Vec<String> = Vec::new(); 

    for (i, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => panic!("Bad input on line {}: {}", i, e)
        };

        let parts: Vec<&str> = line.split(' ').collect();

        // New ocmmand
        if parts[0] == "$" && parts.len() >= 2 {
            // Parse previous command if exists
            if command.len() > 0 {
                let cmd = Command::new(&command);
                command.clear();

                parse_command(&cmd)
            }
        }
        command.push(line.clone());

        // Solve Part 1
        result1 += 1;

        // Solve Part 2
        result2 += 1;
    }

    // print result
    println!("Answer part one: {}", result1);
    println!("Answer part two: {}", result2);
}
