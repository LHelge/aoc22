use std::{fs};
use nom::{
    IResult,
    multi::separated_list1,
    character::complete::{
        newline,
        i64
    },
    bytes::complete::tag
};

pub mod cave;
use crate::cave::*;

fn coord(s: &str) -> IResult<&str, Coord> {
    let (s, _) = tag("x=")(s)?;
    let (s, x) = i64(s)?;
    let (s, _) = tag(", ")(s)?;
    let (s, _) = tag("y=")(s)?;
    let (s, y) = i64(s)?;

    Ok((s, (x,y)))
}

fn sensor(s: &str) -> IResult<&str, Sensor>{
    let (s, _) = tag("Sensor at ")(s)?;
    let (s, position) = coord(s)?;
    let (s, _) = tag(": closest beacon is at ")(s)?;
    let (s, beacon) = coord(s)?;

    Ok((s, Sensor {position, beacon}))
}

fn sensors(s: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list1(
        newline,
        sensor
    )(s)
}

fn part1(input: &str, row: i64) -> usize {
    // Parse input
    let (_, sensors) = sensors(input).expect("Bad input");
    let cave = Cave::new(&sensors);


    cave.no_beacon_position_row(row)
}


fn part2(input: &str, top_left: Coord, bottom_right: Coord) -> u64 {
    let (_, sensors) = sensors(input).expect("Bad input");
    let cave = Cave::new(&sensors);

    let beacon = cave.find_free_spot(&top_left, &bottom_right).unwrap();

    (beacon.0 * 4_000_000 + beacon.1) as u64
}


fn main() {
    let input_ex = fs::read_to_string("input_example.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();


    println!("Part1:");
    println!("Example={}", part1(input_ex.as_str(), 10));
    println!("main={}", part1(input.as_str(), 2000000));

    println!("Part2:");
    println!("Example={}", part2(input_ex.as_str(), (0,0), (20, 20)));
    println!("main={}", part2(input.as_str(), (0,0), (4_000_000, 4_000_000)));
}