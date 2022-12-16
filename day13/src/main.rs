use std::fs;
use std::cmp::Ordering;
use nom::{
    IResult
};

#[derive(Debug, PartialEq)]
enum Data {
    Integer(u32),
    List(Vec<Data>)
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // todo
        Some(Ordering::Greater)
    }
}

struct Pair {
    left: Data,
    right: Data
}



fn parse_packet(s: &str) -> IResult<&str, Data> {
    Result(s, Data::Integer(1))
}


fn parse_pair(s: &str) -> IResult<&str, Pair> {
    let (s, left) = parse_packet(s);
    let (s, )
}


fn parse_input(s: &str) -> IResult<&str, Vec<Pair>> {

}

fn part1(input: &str) -> usize {
    let (_, pairs) = parse_input(input);

    pairs.len()
}

fn part2(input: &str) -> usize {
    input.len()
}



fn main() {
    let input_ex = fs::read_to_string("input_example.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();


    println!("Part1:");
    println!("Example={}", part1(input_ex.as_str()));
    println!("main={}", part1(input.as_str()));

    println!("Part2:");
    println!("Example={}", part2(input_ex.as_str()));
    println!("main={}", part2(input.as_str()));
}