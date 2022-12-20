use std::collections::{HashMap, BTreeSet};
use nom::{
    IResult,
    multi::separated_list1,
    character::complete::{
        newline,
        u64,
        alpha1
    },
    bytes::complete::tag,
    branch::alt
};



#[derive(Debug, Clone)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: u64,
    tunnels: Vec<&'a str>
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct State<'a> {
    position: &'a str,
    time_left: u32,
    opened: BTreeSet<&'a str>
}


// Parse a specific valve using nom
fn valve(s: &str) -> IResult<&str, Valve> {
    let (s, _) = tag("Valve ")(s)?;
    let (s, name) = alpha1(s)?;
    let (s, _) = tag(" has flow rate=")(s)?;
    let (s, flow_rate) = u64(s)?;
    let (s, _) = alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve ")
    ))(s)?;
    let (s, tunnels) = separated_list1(tag(", "), alpha1)(s)?;

    Ok((s, Valve {name, flow_rate, tunnels }))
}


// Parse all valves using nom
fn valves(s: &str) -> IResult<&str, Vec<Valve>> {
    separated_list1(
        newline,
        valve
    )(s)
}


// Recursive search algorithm with a HashMap for caching state
fn max_pressure_released<'a>(state: State<'a>, valves: &HashMap<&str, Valve<'a>>, cache: &mut HashMap<State<'a>, u64>) -> u64 {
    // No time left
    if state.time_left == 0 {
        return 0;
    }

    // Check cache if this state has already been 
    if let Some(max_pressure) = cache.get(&state) {
        return *max_pressure;
    }

    let mut max_left_to_release = 0;

    // Don't run around if all working valves are already opened
    if state.opened.len() <= valves.iter().filter(|(_,v)|v.flow_rate > 0).count() {
        
        let valve = valves.get(state.position).unwrap();

        // Investigate all possible moves
        let max_move = valve.tunnels
            .iter()
            .map(|v| {
                max_pressure_released(
                    State {
                        position: v, 
                        time_left: state.time_left-1,
                        opened: state.opened.clone()
                    }, 
                    valves,
                    cache
                )
            })
            .max()
            .unwrap();
    
        // Investigate open
        let mut max_open = 0;
        if valve.flow_rate > 0 && !state.opened.contains(state.position) {
            let mut opened = state.opened.clone();
            opened.insert(state.position);
            max_open = max_pressure_released(
                State {
                    position: state.position,
                    time_left: state.time_left - 1,
                    opened: opened
                }, 
                valves,
                cache
            );
        }

        max_left_to_release = max_move.max(max_open);
    }

    // Calculate pressure released this time step
    let pressure_released_now = state.opened
        .iter()
        .map(|v| {
            valves.get(v).unwrap().flow_rate
        })
        .sum::<u64>();

    // Store result in cache if needed again
    cache.insert(state, pressure_released_now + max_left_to_release);

    pressure_released_now + max_left_to_release
}


// Solve part1
fn part1(input: &str) -> u64 {
    let valves = valves(input).unwrap().1;
    let valves: HashMap<&str, Valve> = valves
        .iter()
        .map(|v|(v.name, v.clone()))
        .collect();

    //dbg!(&valves);

    let mut cache: HashMap<State, u64> = HashMap::new();

    max_pressure_released(
        State {
            position: "AA",
            time_left: 30,
            opened: BTreeSet::new()
        }, 
        &valves,
        &mut cache
    )
}

fn main() {
    let input_ex = include_str!("../input_example.txt");
    let input = include_str!("../input.txt");

    println!("Part1:");
    println!("Example={}", part1(input_ex));
    println!("main={}", part1(input));

    //println!("Part2:");
    //println!("Example={}", part2(input_ex.as_str()));
    //println!("main={}", part2(input.as_str()));
}
