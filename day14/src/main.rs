use std::{fs};
use nom::{
    IResult,
    sequence::separated_pair,
    multi::separated_list1,
    character::complete::{
        newline,
        u32
    },
    bytes::complete::tag, Parser
};


type Coord = (usize, usize);
type Path = Vec<Coord>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Rock,
    Air,
    Sand
}

#[derive(Debug)]
struct Cave {
    top_left: Coord,
    bottom_right: Coord,
    entry: Coord,
    tiles: Vec<Tile>
}

impl Cave {
    // Create new cave based on path vector
    fn new(paths: &Vec<Path>, entry: Coord) -> Self {
        let mut cave = Self {
            top_left: (usize::MAX, 0), 
            bottom_right: (usize::MIN, usize::MIN),
            entry,
            tiles: vec![]
        };

        // Find edges        
        for path in paths {
            for coord in path {
                cave.top_left.0 = cave.top_left.0.min(coord.0);
                cave.top_left.1 = cave.top_left.1.min(coord.1);
                cave.bottom_right.0 = cave.bottom_right.0.max(coord.0);
                cave.bottom_right.1 = cave.bottom_right.1.max(coord.1);
            }
        }

        // Allocate enough space in Vector to store 2D cave linearly
        let size = cave.coord_to_idx(cave.bottom_right);
        cave.tiles.extend((0..=size).map(|_|Tile::Air));

        // Add paths
        for path in paths {
            for i in 1..path.len() {
                let from = (path[i-1].0.min(path[i].0), path[i-1].1.min(path[i].1));
                let to = (path[i-1].0.max(path[i].0), path[i-1].1.max(path[i].1));

                if from.0 == to.0 {
                    // Vertical
                    for y in from.1..=to.1 {
                        cave.set((from.0,y), Tile::Rock).expect("Bad input");
                    }
                } else {
                    //horizontal
                    for x in from.0..=to.0 {
                        cave.set((x,from.1), Tile::Rock).expect("Bad input");
                    }
                }
            }
        }

        cave
    }

    // Transform a coordinate to index in the underlying vector storage
    fn coord_to_idx(&self, coord: Coord) -> usize {
        self.top_left.0.abs_diff(self.bottom_right.0+1) * 
        self.top_left.1.abs_diff(coord.1) + 
        self.top_left.0.abs_diff(coord.0)
    }

    // Check if a coordinate is inside the cave
    fn is_inside(&self, coord: Coord) -> bool {
        if (self.top_left.0..=self.bottom_right.0).contains(&coord.0) &&
           (self.top_left.1..=self.bottom_right.1).contains(&coord.1) {
            return true;
        }
        false
    }

    // Set a specific cordinate tile
    fn set(&mut self, coord: Coord, tile: Tile) -> Result<(), &'static str> {
        if self.is_inside(coord) {
            let idx = self.coord_to_idx(coord);

            if idx < self.tiles.len() {
                self.tiles[idx] = tile;
                return Ok(())
            }
        }
        Err("Coord not inside cave!")
    }

    // Get a specific coordinate tile
    fn get(&self, coord: Coord) -> Option<Tile> {
        if self.is_inside(coord) {
            let idx = self.coord_to_idx(coord);

            if idx < self.tiles.len() {
                return Some(self.tiles[idx]);
            }
        }
        None
    }

    // Check if a tile is free == Air
    fn is_free(&self, coord: Coord) -> bool {
        match self.get(coord) {
            Some(t) => t == Tile::Air,
            None => true
        }
    }

    // Add one unit of sand and let fall until it comes to rest
    fn add_sand(&mut self) -> bool {
        let mut coord = self.entry;

        // Entry is blocked
        if !self.is_free(coord) {
            return false
        }

        // Loop as long as there is somewhere to fall
        while self.is_free((coord.0, coord.1+1)) ||
              self.is_free((coord.0-1, coord.1+1)) ||
              self.is_free((coord.0+1, coord.1+1)) {

            if self.is_free((coord.0, coord.1+1)) {
                // Prio1: Down
                coord.1 += 1;
            }
            else if self.is_free((coord.0-1, coord.1+1)) {
                // Prio2: Down-Left
                coord.0 -= 1;
                coord.1 += 1;
            }
            else if self.is_free((coord.0+1, coord.1+1)) {
                // Prio3: Down right
                coord.0 += 1;
                coord.1 += 1;
            }

            if !self.is_inside(coord) {
                return false
            }
        }

        // Set the final coord to Sand
        match self.set(coord, Tile::Sand) {
            Ok(_) => (),
            Err(_) => return false
        }
        return true
    }

    // Count how many units of sand the cave contains
    fn count_sand(&self) -> u32 {
        self.tiles.iter().filter(|&t|*t==Tile::Sand).count() as u32
    }

    // Print the cave
    fn print(&self) {
        for y in self.top_left.1..=self.bottom_right.1 {
            print!("{:>3} ", y);
            for x in self.top_left.0..=self.bottom_right.0 {
                if self.entry.0 == x && self.entry.1 == y {
                    print!("+")
                } else {
                    match self.get((x, y)).unwrap() {
                        Tile::Air => print!("."),
                        Tile::Rock => print!("#"),
                        Tile::Sand => print!("o")
                    }
                }
            }
            println!("");
        }
    }
}


fn paths(s: &str) -> IResult<&str, Vec<Path>> {
    separated_list1(
        newline,
        separated_list1(
            tag(" -> "),
            separated_pair(
                u32, 
                tag(","),
                u32
            ).map(|(x, y)| (x as usize, y as usize))
        )
    )(s)
}


fn part1(input: &str) -> u32 {
    // Parse input
    let (_, paths) = paths(input).unwrap();

    // Create cave
    let mut cave = Cave::new(&paths, (500,0));

    // Add sand until it falls out
    while cave.add_sand() {}

    // Print cave and return the number of sand units
    cave.print();
    cave.count_sand()
}


fn part2(input: &str) -> u32 {
    // Pars input
    let (_, mut paths) = paths(input).unwrap();

    // Find the lowes unit
    let max_y = paths
        .iter()
        .map(|p| {
            p.iter().map(|s| s.1).max().unwrap()
        }).max().unwrap();

    // Add floor 2 units below
    paths.push(vec![(300,max_y+2), (700, max_y+2)]);

    // Create cave
    let mut cave = Cave::new(&paths, (500,0));

    // Add sand until entry is blocked
    while cave.add_sand() {}

    // Print cave and return the ammount of sand
    cave.print();
    cave.count_sand()
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
