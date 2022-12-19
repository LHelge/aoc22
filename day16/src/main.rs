use core::fmt;
use std::{fs, collections::{HashMap, HashSet}};
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

#[derive(Debug, Clone, Copy)]
enum ValveState {
    Closed,
    Open(u64)
}

#[derive(Debug, Clone)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: u64,
    tunnels: Vec<&'a str>,
    state: ValveState
}


#[derive(Debug)]
enum Action<'a> {
    Goto(&'a str),
    Open(&'a str)
}

fn valve(s: &str) -> IResult<&str, Valve> {
    let (s, _) = tag("Valve ")(s)?;
    let (s, name) = alpha1(s)?;
    let (s, _) = tag(" has flow rate=")(s)?;
    let (s, flow_rate) = u64(s)?;
    let (s, _) = tag("; tunnels lead to ")(s)?;
    let (s, _) = alt((tag("valves "), tag("valve ")))(s)?;
    let (s, tunnels) = separated_list1(tag(", "), alpha1)(s)?;

    Ok((s, Valve {name, flow_rate, tunnels, state: ValveState::Closed }))
}

fn valves(s: &str) -> IResult<&str, Vec<Valve>> {
    separated_list1(
        newline,
        valve
    )(s)
}

fn calc_pressure_release(valves: &HashMap<&str, Valve>, actions: &Vec<Action>) -> u64 {
    let mut pressure_release: u64 = 0;
    let mut open_valves: HashSet<&str> = HashSet::new();

    // let mut current_valve = "AA";
    for (i, action) in actions.iter().enumerate() {
        println!("\n== Minute {} ==", i+1);
        
        let release = valves.iter()
            .filter(|(k,_)|open_valves.contains(**k))
            .map(|(_, v)|v.flow_rate)
            .sum::<u64>();
        
        println!("Open valves: {:?}, releasing {} pressure", open_valves, release);
        pressure_release += release;

        match action {
            Action::Open(valve) => {
                // TODO: validate open
                open_valves.insert(valve);
                println!("You open valve {}", valve);
            },
            Action::Goto(valve) => {
                // TODO: validate move
                println!("You move to valve {}", valve);
            }
        };
    }

    pressure_release
}

fn part1(input: &str) -> u64 {
    let valves = valves(input).unwrap().1;
    let valves: HashMap<&str, Valve> = valves
        .iter()
        .map(|v|(v.name, v.clone()))
        .collect();

    dbg!(&valves);

    let mut actions: Vec<Action> = vec![];
    actions.push(Action::Goto("DD"));
    actions.push(Action::Open("DD"));
    actions.push(Action::Goto("CC"));
    actions.push(Action::Goto("BB"));
    actions.push(Action::Open("BB"));
    actions.push(Action::Goto("AA"));
    actions.push(Action::Goto("II"));
    actions.push(Action::Goto("JJ"));
    actions.push(Action::Open("JJ"));

    calc_pressure_release(&valves, &actions)
}

fn main() {
    let input_ex = fs::read_to_string("input_example.txt").unwrap();
    //let input = fs::read_to_string("input.txt").unwrap();

    println!("Part1:");
    println!("Example={}", part1(input_ex.as_str()));
    //println!("main={}", part1(input.as_str()));

    //println!("Part2:");
    //println!("Example={}", part2(input_ex.as_str()));
    //println!("main={}", part2(input.as_str()));
}
