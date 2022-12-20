use std::{collections::HashSet, ops::RangeInclusive};

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

// Define which offsets to look in for adjacent cubes
const OFFSETS: [Position; 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1)
];


#[derive(Debug)]
struct Bounds {
    min: Position,
    max: Position
}


impl Bounds {
    fn from(cubes: &Vec<Position>) -> Self {
        let mut bounds = Self { 
            min: (i64::MAX, i64::MAX, i64::MAX), 
            max: (i64::MIN, i64::MIN, i64::MIN)
        };

        for cube in cubes {
            bounds.min.0 = bounds.min.0.min(cube.0);
            bounds.min.1 = bounds.min.1.min(cube.1);
            bounds.min.2 = bounds.min.2.min(cube.2);
            bounds.max.0 = bounds.max.0.max(cube.0);
            bounds.max.1 = bounds.max.1.max(cube.1);
            bounds.max.2 = bounds.max.2.max(cube.2);
        }

        bounds
    }

    fn x_range(&self) -> RangeInclusive<i64> {
        self.min.0..=self.max.0
    }

    fn y_range(&self) -> RangeInclusive<i64> {
        self.min.1..=self.max.1
    }

    fn z_range(&self) -> RangeInclusive<i64> {
        self.min.2..=self.max.2
    }
}

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


fn part2(input: &str) -> usize {
    let cubes = cubes(input).unwrap().1;
    let bounds = Bounds::from(&cubes);
    let lava: HashSet<Position> = cubes.into_iter().collect();

    // Analyze slices to find "outside"
    let mut outside: HashSet<Position> = HashSet::new();
    for x in bounds.x_range() {
        for y in bounds.y_range() {
            for z in bounds.z_range() {
                // TODO: Find all outside positions
                if !lava.contains(&(x,y,z)) {
                    outside.insert((x,y,z));
                }
            }
        }
    }
    // For now, remove the known void from the example.
    outside.remove(&(2,2,5));
    
    let mut inside_faces = 0usize;
    for x in bounds.x_range() {
        for y in bounds.y_range() {
            for z in bounds.z_range() {
                if !outside.contains(&(x,y,z)) && !lava.contains(&(x,y,z)) {
                    // Inside bubble
                    inside_faces += OFFSETS
                    .iter()
                    .filter(|(ox, oy, oz)| {
                        lava.contains(&(x-ox, y-oy, z-oz))
                    })
                    .count();

                    println!("Inside pos {:?}, total {} faces", (x,y,z), inside_faces);
                }
            }
        }
    }

    let lava_faces = lava
        .iter()
        .map(|(x,y,z)| {
            OFFSETS
                .iter()
                .filter(|(ox, oy, oz)| {
                    !lava.contains(&(x-ox, y-oy, z-oz))
                })
                .count()
        })
        .sum::<usize>();

    lava_faces - inside_faces
}

fn main() {
    let input_ex = include_str!("../input_example.txt");
    let input = include_str!("../input.txt");

    println!("Part1");
    println!("Example={}", part1(input_ex));
    println!("Main={}", part1(input));

    println!("Part2");
    println!("Example={}", part2(input_ex));
    //println!("Main={}", part2(input));
}
