use std::{fs};

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32)
}

impl Instruction {
    fn parse(s: &str) -> Option<Instruction> {
        let parts: Vec<&str> = s.split(' ').collect();
        
        match parts[0] {
            "noop" => Some(Instruction::Noop),
            "addx" => Some(Instruction::AddX(parts[1].parse().unwrap())),
            _ => None
        }
    }
}


fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instr: Vec<Instruction> = vec![];

    for line in input.lines() {
        instr.push(Instruction::parse(line).unwrap());
    }

    instr
}

fn calc_reg_cycles(instructions: &Vec<Instruction>) -> Vec<i32> {
    let mut reg = 1;
    let mut cycles: Vec<i32> = vec![];

    for i in instructions {
        match i {
            Instruction::Noop => cycles.push(reg),
            Instruction::AddX(x) => {
                cycles.push(reg);
                cycles.push(reg);
                reg += x;
            }
        }
    }

    cycles
}

fn part1(cycles: &Vec<i32>) -> i32 {
    let interesting: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let mut sum = 0;
    for cycle in interesting {
        sum += cycle as i32 * cycles[cycle-1];
    }

    sum
}

fn draw_part2(cycles: &Vec<i32>) {
    for row in 0..6 {
        for col in 0..40 {
            let reg = cycles[40 * row + col];
            if (col as i32)-1 <= reg && (col as i32)+1 >= reg {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n")
    }
}

fn main() {
    let input_ex = fs::read_to_string("input_example.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();

    let instr_ex = parse_instructions(&input_ex);
    let instr = parse_instructions(&input);

    let cycles_ex = calc_reg_cycles(&instr_ex);
    let cycles = calc_reg_cycles(&instr);

    println!("Answer part one:");
    println!("Example={}", part1(&cycles_ex));
    println!("Main={}", part1(&cycles));

    println!("Answer part one:");
    println!("Example:");
    draw_part2(&cycles_ex);
    println!("Main:");
    draw_part2(&cycles);
}
