use std::{fs, collections::{BinaryHeap, BTreeMap}};



#[derive(Debug)]
struct Map<const M: usize, const N: usize> {
    height: [[u8; M]; N],
    start: (usize, usize),
    end: (usize, usize)
}

impl<const M: usize, const N: usize> Map<M, N> {
    fn from(s: &str) -> Self {
        const A_OFFSET: u8 = 'a' as u8;
        let mut map = Self {height: [[0; M];N], start: (0,0), end: (0,0)};

        for (row, line) in s.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        map.start = (row, col);
                        map.height[row][col] = 'a' as u8 - A_OFFSET;
                    }
                    'E' => {
                        map.end = (row, col);
                        map.height[row][col] = 'z' as u8 - A_OFFSET;
                    }
                    h @ 'a'..='z' => map.height[row][col] = (h) as u8 - A_OFFSET,
                    _ => panic!("Bad input")
                };
            }
        }

        map
    }

    fn get_adjacent(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut adjacent: Vec<(usize, usize)> = vec![];

        // Up
        // Down
        // Left
        // right
        // TODO

        adjacent
    }
}


#[derive(Eq, PartialOrd, Debug)]
struct Node {
    cost: u32,
    pos: (usize, usize)
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

fn part1<const M: usize, const N: usize>(input: &str) -> u32 {

    let mut map: Map<M, N> = Map::from(input);

    println!("{:#?}", map);

    // Dijkstra
    let mut open: BinaryHeap<Node> = BinaryHeap::new();
    let mut closed: BTreeMap<(usize, usize), u32> = BTreeMap::new();
    let mut done = false;

    open.push(Node {cost: 0, pos: map.start });

    while done {
        let current = open.pop().unwrap();

        if current.pos == map.end {
            done = true; 
        } else {
            // Add adjacent nodes if possible
            for pos in map.get_adjacent(current.pos) {
                open.push(Node {cost: current.cost+1, pos});
            }
        }
        
        // The current are done
        closed.insert(current.pos, current.cost);
    }

    
    println!("{:#?}", open);
    println!("{:#?}", closed);

    *closed.get(&map.end).unwrap()
}


fn main() {
    let input_ex = fs::read_to_string("input_example.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();


    println!("Part1:");
    println!("Example={}", part1::<8, 5>(input_ex.as_str()));
}
