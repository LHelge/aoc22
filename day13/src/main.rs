use std::fs;
use std::cmp::Ordering;
use nom::{
    IResult,
    sequence::{
        separated_pair,
        delimited
    },
    multi::separated_list0,
    multi::separated_list1,
    character::complete::{
        newline,
        u32
    },
    branch::alt,
    bytes::complete::tag, Parser
};

#[derive(Debug, Clone, Eq)]
enum Data {
    Integer(u32),
    List(Vec<Data>)
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Data::Integer(s), Data::Integer(o)) => s.cmp(o),
            (Data::Integer(s), Data::List(o)) => vec![Data::Integer(*s)].cmp(o),
            (Data::List(s), Data::Integer(o)) => s.cmp(&vec![Data::Integer(*o)]),
            (Data::List(s), Data::List(o)) => s.cmp(o)
        }
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Data::Integer(s), Data::Integer(o)) => s.eq(o),
            (Data::Integer(s), Data::List(o)) => vec![Data::Integer(*s)].eq(o),
            (Data::List(s), Data::Integer(o)) => s.eq(&vec![Data::Integer(*o)]),
            (Data::List(s), Data::List(o)) => s.eq(o)
        }
    }
}

#[derive(Debug)]
struct Pair {
    left: Data,
    right: Data
}

impl Pair {
    fn right_order(&self) -> bool {
        self.left < self.right
    }
}

fn integer(s: &str) -> IResult<&str, Data> {
    let (s, int) = u32(s)?;

    Ok((s, Data::Integer(int)))
}


fn list(s: &str) -> IResult<&str, Vec<Data>> {
    delimited(
        tag("["),
        separated_list0(tag(","), packet),
        tag("]")
    )(s)
}

fn packet(s: &str) -> IResult<&str, Data> {
    alt((
        list.map(|l|Data::List(l)),
        integer
    ))(s)
}


fn pairs(s: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"), 
        separated_pair(
            packet, 
            newline, 
            packet
        ).map(|(left, right)| Pair { left, right })
    )(s)
}

fn all(s: &str) -> IResult<&str, Vec<Data>> {
    separated_list1(
        alt((tag("\n\n"), tag("\n"))),
        packet
    )(s)
}

fn part1(input: &str) -> usize {
    let (_, pairs) = pairs(input).unwrap();

    pairs
        .iter()
        .enumerate()
        .map(
            |(i, p)| {
                match p.right_order() {
                    true => i+1,
                    false => 0
                }
            })
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    let mut packets = all(input).unwrap().1;
    
    let divider1 = Data::List(vec![Data::List(vec![Data::Integer(2)])]);
    let divider2 = Data::List(vec![Data::List(vec![Data::Integer(6)])]);
    packets.push(divider1.clone());
    packets.push(divider2.clone());
    
    packets.sort();
    //println!("{:#?}", packets);


    let idx1 = packets
        .iter()
        .enumerate()
        .find(|(_ , p)| **p == divider1)
        .map(|(i, _)| i+1);

    let idx2 = packets
        .iter()
        .enumerate()
        .find(|(_ , p)| **p == divider2)
        .map(|(i, _)| i+1);

    
    idx1.unwrap() * idx2.unwrap()
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