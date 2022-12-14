use std::fs;

enum MapPoint {
    Start,
    End,
    Height(u32)
}

enum Direction {
    Up,
    Right,
    Down,
    Left
}

struct Map<const M: usize, const N: usize> {
    points: [[MapPoint; N]; M],
    start: (usize, usize),
    end: (usize, usize)
}


fn main() {
    let input_ex = fs::read_to_string("input_example.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();


    println!("Hello, world!");
}
