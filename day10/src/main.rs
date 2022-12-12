use std::{fs};

extern crate nom;
use nom::{
    IResult,
    character::complete::{
        newline,
        not_line_ending, alpha1, multispace0, digit0
    },
    multi::separated_list1,
    branch::alt, error::Error, combinator::map_res, sequence::separated_pair
};


enum Instruction {
    Noop,
    AddX(i32)
}

impl Instruction {
    fn parse<'a>((op, data): (&'a str, &'a str)) -> &Result<Instruction, Error<&'a str>> {
        Ok(Instruction::Noop)
    }
}

fn parse_instructions(s: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(
        newline,
        map_res(
            separated_pair(alpha1, multispace0, digit0), 
            Instruction::parse
        )
    )(s)
}


fn part1(input: &str) -> i32 {

    let instructions = parse_instructions(input).unwrap().1;

    instructions.len() as i32
}

fn main() {
    let input_ex = fs::read_to_string("input_example.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Answer part one:");
    println!("Example={}", part1(input_ex.as_str()));
    println!("Main={}", part1(input.as_str()));
}
