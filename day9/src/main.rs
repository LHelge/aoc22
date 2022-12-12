use std::{fs};

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
    multi::separated_list1,
    error::Error,
    error::ErrorKind
};

enum Direction{
    Up,
    Down,
    Right,
    Left,
}

struct Motion {
    dir: Direction,
    dist: u32
}

impl Motion {
    fn parse<'a>((dir, dist): (&'a str, &'a str)) -> Result<Motion, Error<&'a str>> {
        let dist: u32 = dist.parse().unwrap();
        match dir {
            "U" => Ok(Motion {dir: Direction::Up, dist}),
            "D" => Ok(Motion {dir: Direction::Down, dist}),
            "R" => Ok(Motion {dir: Direction::Right, dist}),
            "L" => Ok(Motion {dir: Direction::Left, dist}),
            _ => Err(Error::new(dir, ErrorKind::MapRes))
        }

        
    }
}

fn parse_motions(s: &str) -> IResult<&str, Vec<Motion>> {
    separated_list1(
        newline,
        map_res(
            separated_pair(alpha1, char(' '), digit1),
            Motion::parse
        )
    )(s)
}

#[derive(PartialEq, Clone, Copy)]
struct Knot {
    x: i32,
    y: i32
}


struct Rope<const N: usize> {
    knots: [Knot; N]
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Rope { knots: [Knot{x: 0, y: 0}; N]}
    }

    fn move_head(&mut self, dir: &Direction) {
        // Move head in the direction
        match dir {
            Direction::Up => self.knots[0].y += 1,
            Direction::Down => self.knots[0].y -= 1,
            Direction::Right => self.knots[0].x += 1,
            Direction::Left => self.knots[0].x -= 1,
        }

        // Move all following knots according to the rules
        for i in 1..N {
            // Predecessor is above
            if (self.knots[i-1].y - self.knots[i].y) > 1 {
                // Move up
                self.knots[i].y += 1;

                // Move diagonally if needed
                if self.knots[i-1].x > self.knots[i].x {self.knots[i].x += 1}        // Move right
                else if self.knots[i-1].x < self.knots[i].x {self.knots[i].x -= 1}   // Move left
            }

            // Predecessor is below
            if (self.knots[i-1].y - self.knots[i].y) < -1 {
                // Move down
                self.knots[i].y -= 1;

                // Move diagonally if needed
                if self.knots[i-1].x > self.knots[i].x {self.knots[i].x += 1}        // Move right
                else if self.knots[i-1].x < self.knots[i].x {self.knots[i].x -= 1}   // Move left
            }

            // Predecessor is to the right
            if (self.knots[i-1].x - self.knots[i].x) > 1 {
                // Move right
                self.knots[i].x += 1;

                // Move diagonally if needed
                if self.knots[i-1].y > self.knots[i].y {self.knots[i].y += 1}        // Move up
                else if self.knots[i-1].y < self.knots[i].y {self.knots[i].y -= 1}   // Move down
            }

            // Predecessor is to the left
            if (self.knots[i-1].x - self.knots[i].x) < -1 {
                // Move left
                self.knots[i].x -= 1;

                // Move diagonally if needed
                if self.knots[i-1].y > self.knots[i].y {self.knots[i].y += 1}        // Move up
                else if self.knots[i-1].y < self.knots[i].y {self.knots[i].y -= 1}   // Move down
            }
        }
    }

    fn get_tail(&self) -> Option<&Knot> {
        self.knots.last()
    }
}


// Find answer to Part 1
fn part1(motions: &Vec<Motion>) -> usize {
    // Create a vector to store all locations visited by the tail
    let mut visited: Vec<Knot> = vec![];

    // Create points to store the head and tail location
    let mut rope: Rope<2> = Rope::new();
    
    for motion in motions {
        for _m in 0..motion.dist {
            rope.move_head(&motion.dir);

            // Store all unique locations visited by the tail
            let tail = rope.get_tail().unwrap();
            if !visited.contains(&tail) {
                visited.push(tail.clone());
            }
        }
    }

    visited.len()
}

// Find answer to Part 2
fn part2(motions: &Vec<Motion>) -> usize {
    // Create a vector to store all locations visited by the tail
    let mut visited: Vec<Knot> = vec![];

    // Create points to store the head and tail location
    let mut rope: Rope<10> = Rope::new();
    
    for motion in motions {
        for _m in 0..motion.dist {
            rope.move_head(&motion.dir);

            // Store all unique locations visited by the tail
            let tail = rope.get_tail().unwrap();
            if !visited.contains(&tail) {
                visited.push(tail.clone());
            }
        }
    }

    visited.len()
}

fn main() {
    // Input data
    let input_example1 = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    let input_example2 = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");

    // Use nom to parse motions from the input
    let motions_ex1 = parse_motions(input_example1).unwrap().1;
    let motions_ex2 = parse_motions(input_example2).unwrap().1;
    let motions = parse_motions(input.as_str()).unwrap().1;
    
    // print result
    println!("Answer part one:");
    println!("Example={}", part1(&motions_ex1));
    println!("Main={}", part1(&motions));
    println!("\nAnswer part two:");
    println!("Example={}", part2(&motions_ex2));
    println!("Main={}", part2(&motions));

}

