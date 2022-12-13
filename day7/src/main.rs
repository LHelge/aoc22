use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
enum Command<'a> {
    Cd(Cd<'a>),
    Ls(Vec<FsObj<'a>>)
}

#[derive(Debug, Clone)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str)
}

#[derive(Debug, Clone)]
enum FsObj<'a> {
    File(&'a str, usize),
    Dir(&'a str)
}

impl<'a> Command<_> {
    fn new<'a>(input: &'a Vec<String>) -> Self {
        let parts: Vec<&str> = input[0].split(' ').collect();
        
        match parts[1] {
            "cd" => match parts[2] {
                "/" => Command::Cd(Cd::Root),
                ".." => Command::Cd(Cd::Up),
                dir => Command::Cd(Cd::Down(dir))
            },
            "ls" => {
                Command::Ls(vec![])
            },
            _ => panic!("Unknown command")
        }
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
    let mut commands: Vec<Command> = vec![];

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

                commands.push(cmd.clone());
            }
        }
        command.push(line.clone());

        // Solve Part 1
        result1 += 1;

        // Solve Part 2
        result2 += 1;
    }

    let mut cwd: Vec<&str> = vec![];
    let mut dirs: HashMap<&str, usize> = HashMap::new();


    println!("{:#?}", commands);

    // print result
    println!("Answer part one: {}", result1);
    println!("Answer part two: {}", result2);
}
