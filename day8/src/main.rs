use std::fs::File;
use std::io::{BufRead, BufReader};


// const FILENAME: &str = "input_example.txt";
// const WIDTH: usize = 5; // Example
// const HEIGHT: usize = 5; // Example
const FILENAME: &str = "input.txt";
const WIDTH: usize = 99; // Real input
const HEIGHT: usize = 99; // Real input

fn find_visible(heights: &[[u8; WIDTH]; HEIGHT]) -> [[bool; WIDTH]; HEIGHT] {
    let mut visible = [[false; WIDTH]; HEIGHT];

    // Check rows
    for row in 0..HEIGHT {
        // All edge trees are visible
        visible[row][0] = true;
        visible[row][WIDTH-1] = true;

        // Check from left
        let mut highest = heights[row][0];
        for col in 1..WIDTH-1 {
            if heights[row][col] > highest {
                visible[row][col] = true;
                highest = heights[row][col];

                if highest >= 9 {
                    break;
                }
            }
        }

        // Check from right
        let mut highest = heights[row][WIDTH-1];
        for col in (1..WIDTH-1).rev() {
            if heights[row][col] > highest {
                visible[row][col] = true;
                highest = heights[row][col];

                if highest >= 9 {
                    break;
                }
            }
        }
    }

    // Check columns
    for col in 0..WIDTH {
        // All edge trees are visible
        visible[0][col] = true;
        visible[HEIGHT-1][col] = true;

        // Check from top
        let mut highest = heights[0][col];
        for row in 1..HEIGHT-1 {
            if heights[row][col] > highest {
                visible[row][col] = true;
                highest = heights[row][col];

                if highest >= 9 {
                    break;
                }
            }
        }

        // Check from bottom
        let mut highest = heights[HEIGHT-1][col];
        for row in (1..HEIGHT-1).rev() {
            if heights[row][col] > highest {
                visible[row][col] = true;
                highest = heights[row][col];

                if highest >= 9 {
                    break;
                }
            }
        }
    }

    visible
}


fn find_scenic_score(heights: &[[u8; WIDTH]; HEIGHT]) -> [[u32; WIDTH]; HEIGHT] {
    let mut scenic_score = [[0 as u32; WIDTH]; HEIGHT];

    // Check all trees not at the edge since one distance will be 0, multiplying with other will still be 0
    for row in 1..HEIGHT-1 {
        for col in 1..WIDTH-1 {
            // Check right
            let mut right: u32 = 0;
            for i in col+1..WIDTH {
                right += 1;

                if heights[row][i] >= heights[row][col] {
                    break;
                }
            }

            // Check left
            let mut left: u32 = 0;
            for i in (0..col).rev() {
                left += 1;
                
                if heights[row][i] >= heights[row][col] {
                    break;
                }
            }

            // Check down
            let mut down: u32 = 0;
            for i in row+1..HEIGHT {
                down += 1;

                if heights[i][col] >= heights[row][col] {
                    break;
                }
            }

            // Check up
            let mut up: u32 = 0;
            for i in (0..row).rev() {
                up += 1;
                
                if heights[i][col] >= heights[row][col] {
                    break;
                }
            }

            scenic_score[row][col] = left * right * up * down;
        }
    }

    scenic_score
}


fn main() {
    // Read file
    let file = match File::open(FILENAME) {
        Ok(f) => f,
        Err(e) => panic!("Problem reading file {}: {}", FILENAME, e)
    };
    let reader = BufReader::new(file);

    // Read file
    let mut tree_height = [[0 as u8; WIDTH]; HEIGHT];
    let mut row: usize = 0;
    for (i, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => panic!("Bad input on line {}: {}", i, e)
        };
        
        let mut col = 0;
        for c in line.chars() {
            tree_height[row][col] = String::from(c).parse().unwrap();
            
            col += 1;
        }
        row += 1;
    }
    
    // Part 1
    let tree_visible = find_visible(&tree_height);
    let mut visible: usize = 0;
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            visible += tree_visible[row][col] as usize; // Assume true as usize == 1
        }
    }
    println!("Answer part one: {}", visible);

    // Part 2
    let scenic_score = find_scenic_score(&tree_height);
    let mut highest: (usize, usize) = (0, 0);
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            if scenic_score[row][col] > scenic_score[highest.0][highest.1] {
                highest = (row, col);
            }
        }
    }
    println!("Answer part two: {} at ({},{})", scenic_score[highest.0][highest.1], highest.0, highest.1);
}