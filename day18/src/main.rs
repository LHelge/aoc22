use std::collections::HashSet;

use nom::{
    IResult,
    multi::separated_list1,
    character::complete::{
        newline,
        char,
        i64
    }, Parser
};

type Position = (i64, i64, i64);

// Parse cubes from text using nom
fn cubes(s: &str) -> IResult<&str, Vec<Position>> {
    separated_list1(
        newline,
        separated_list1(
            char(','),
            i64
        ).map(|p|(p[0], p[1], p[2]))
    )(s)
}

// Find surface area of lava ball
fn part1(input: &str) -> usize {
    // Parse cubes into HashSet<Position>
    let cubes: HashSet<Position> = cubes(input).unwrap().1.into_iter().collect();

    // Define which offsets to look in for adjacent cubes
    const OFFSETS: [Position; 6] = [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1)
    ];

    // Calculate surface area
    cubes
        .iter()
        .map(|(x,y,z)| {
            OFFSETS
                .iter()
                .filter(|(ox, oy, oz)| {
                    !cubes.contains(&(x-ox, y-oy, z-oz))
                })
                .count()
        })
        .sum::<usize>()
}


fn main() {
    let input_ex = include_str!("../input_example.txt");
    let input = include_str!("../input.txt");

    println!("Part1");
    println!("Example={}", part1(input_ex));
    println!("Main={}", part1(input));
}
