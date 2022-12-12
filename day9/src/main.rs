use std::{fs, fmt::Error};

extern crate nom;
use nom::{
    IResult,
    character::complete::{
        char,
        alpha1,
        digit1,
        newline,
    },
    combinator::map_res,
    sequence::separated_pair,
    multi::separated_list1
};

#[derive(Debug)]
enum Motion{
    Up(u32),
    Down(u32),
    Right(u32),
    Left(u32),
}

impl Motion {
    fn parse((dir, dist): (&str, &str)) -> Result<Motion, Error> {
        match dir {
            "U" => Ok(Motion::Up(dist.parse().unwrap())),
            "D" => Ok(Motion::Down(dist.parse().unwrap())),
            "R" => Ok(Motion::Right(dist.parse().unwrap())),
            "L" => Ok(Motion::Left(dist.parse().unwrap())),
            _ => panic!("Error!")
        }
    }
}


fn motion(s: &str) -> IResult<&str, Motion> {
    map_res(
        separated_pair(alpha1, char(' '), digit1),
        Motion::parse
    )(s)
}

fn parser(s: &str) -> IResult<&str, Vec<Motion>> {
    separated_list1(newline, motion)(s)
}

fn part1(input: &str) -> u32 {
    let motions = parser(input);

    println!("{:#?}", motions.expect("Parse error").1);

    1
}



fn main() {
    // Input data
    let input_example = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");
    
    // print result
    println!("Answer part one:");
    println!("Example={}", part1(input_example));
    //println!("Main={}", part1(input.as_str()));

}

