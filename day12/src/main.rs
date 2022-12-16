use std::{fs, collections::{BinaryHeap, HashMap}};



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

    fn height(&self, pos: &(usize, usize)) -> Option<u8> {
        if pos.0 < N && pos.1 < M {
            Some(self.height[pos.0][pos.1])
        } else {
            None
        }
    }

    fn get_adjacent(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        const OFFSET: [(i32, i32); 4] = [(-1,0), (1,0), (0,-1), (0,1)];

        OFFSET.iter()
            .map(|p| (pos.0 as i32 + p.0, pos.1 as i32 + p.1))
            .filter(
                |p| {
                    (0..N as i32).contains(&p.0) && (0..M as i32).contains(&p.1)
                })
            .map(|p| (p.0 as usize, p.1 as usize))
            .filter(|p| self.height(pos).unwrap()+1 >= self.height(p).unwrap())
            .collect()
    }
}



#[derive(Eq, Debug)]
struct Node {
    steps: u32,
    pos: (usize, usize)
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

// Use Dijkstras algorithm to find the shortest path on the map
fn find_shortest_path<const M: usize, const N: usize>(map: &Map<M, N>) -> Option<u32> {
    // Dijkstra
    let mut open: BinaryHeap<Node> = BinaryHeap::new();
    let mut closed: HashMap<(usize, usize), u32> = HashMap::new();

    // Add start
    open.push(Node {steps: 0, pos: map.start});

    while let Some(current) = open.pop() {
        // Check if we've found the end
        if current.pos.eq(&map.end) {
            return Some(current.steps);
        }

        // Skip if already visited
        if closed.contains_key(&current.pos) {
            continue;
        }

        // Add adjacent nodes if possible
        for pos in map.get_adjacent(&current.pos) {
            if !closed.contains_key(&pos) {
                open.push(Node {steps: current.steps+1, pos});
            }
        }
        
        // The current are done
        closed.insert(current.pos, current.steps);
    }

    None
}

fn part1<const M: usize, const N: usize>(input: &str) -> Option<u32> {

    let map: Map<M, N> = Map::from(input);

    find_shortest_path(&map)
}

fn part2<const M: usize, const N: usize>(input: &str) -> Option<u32> {

    let mut map: Map<M, N> = Map::from(input);

    let mut shortest = u32::MAX;
    for row in 0..N {
        for col in 0..M {
            let pos = (row, col);
            if map.height(&pos) == Some(0) {
                map.start = pos;
                if let Some(steps) = find_shortest_path(&map) {
                    shortest = shortest.min(steps);
                }
            }
        }
    }

    Some(shortest)
}


fn main() {
    let input_ex = fs::read_to_string("input_example.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();


    println!("Part1:");
    println!("Example={}", part1::<8, 5>(input_ex.as_str()).unwrap());
    println!("main={}", part1::<71, 41>(input.as_str()).unwrap());

    println!("Part2:");
    println!("Example={}", part2::<8, 5>(input_ex.as_str()).unwrap());
    println!("main={}", part2::<71, 41>(input.as_str()).unwrap());
}
