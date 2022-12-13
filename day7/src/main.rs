use std::{fs, collections::HashMap};

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


fn parse_commands(input: &str) -> Vec<Command> {
    let mut commands: Vec<Command> = vec![];
    let mut it = input.lines().peekable();

    loop {
        match it.next() {
            Some(line) => {
                if line.starts_with("$ cd") {
                    match &line[5..] {
                        "/" => commands.push(Command::Cd(Cd::Root)),
                        ".." => commands.push(Command::Cd(Cd::Up)),
                        dir => commands.push(Command::Cd(Cd::Down(dir)))
                    }
                }
                else if line.starts_with("$ ls") {
                    let mut content: Vec<FsObj> = vec![];
                    while it.peek().is_some() && !it.peek().unwrap().starts_with("$ ") {
                        let obj = it.next().unwrap();
                        if obj.starts_with("dir ") {
                            content.push(FsObj::Dir(&obj[5..]));
                        }
                        else {
                            let parts: Vec<&str> = obj.split(' ').collect();
                            content.push(FsObj::File(parts[1], parts[0].parse().unwrap()));
                        }
                    }
                    commands.push(Command::Ls(content));
                }
            }
            None => break
        };
    }

    commands
}


fn calc_sizes(commands: &Vec<Command>) -> HashMap<String, usize> {
    let mut cwd: Vec<&str> = vec![];
    let mut sizes: HashMap<String, usize> = HashMap::new();

    for cmd in commands {
        match cmd {
            Command::Cd(cd) => {
                match cd {
                    Cd::Root => {cwd.clear(); cwd.push("")},
                    Cd::Up => {cwd.pop();},
                    Cd::Down(dir) => cwd.push(dir)
                }
            },
            Command::Ls(files) => {
                let mut dir_size: usize = 0;
                for file in files {
                    if let FsObj::File(.., size) = file {
                        dir_size += size;
                    }
                }

                for i in 0..cwd.len() {
                    sizes.entry(cwd[0..=i].join("/"))
                        .and_modify(|v| *v += dir_size)
                        .or_insert(dir_size);
                }
            }
        }
    }

    sizes
}


fn part1(sizes: &HashMap<String, usize>) -> usize {
    let mut sum: usize = 0;
    for (_, size) in sizes {
        if size <= &100_000 {
            sum += size;
        }
    }

    sum
}


fn part2(sizes: &HashMap<String, usize>) -> usize {
    let total_space: usize = 70_000_000;
    let needed_space: usize = 30_000_000;
    let used_space = sizes.get(&String::from("")).unwrap().to_owned();

    let to_delete = (needed_space + used_space) - total_space;

    let mut smallest_to_delete = used_space;

    for (_, size) in sizes {
        if size >= &to_delete && size < &smallest_to_delete {
            smallest_to_delete = *size;
        }
    }

    smallest_to_delete
}


fn main() {
    let input_ex = fs::read_to_string("input_example.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();

    let commands_ex = parse_commands(&input_ex);
    let commands = parse_commands(&input);

    let sizes_ex = calc_sizes(&commands_ex);
    let sizes = calc_sizes(&commands);


    // print result
    println!("Answer part one:");
    println!("Example={}", part1(&sizes_ex));
    println!("Main={}", part1(&sizes));

    println!("Answer part Two:");
    println!("Example={}", part2(&sizes_ex));
    println!("Main={}", part2(&sizes));
}
