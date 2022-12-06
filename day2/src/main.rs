use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Action {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug)]
enum Outcome {
    Win,
    Loose,
    Draw
}

fn to_action(action: &str) -> Option<Action> {
    match action {
        "A" | "X" => Some(Action::Rock),
        "B" | "Y" => Some(Action::Paper),
        "C" | "Z" => Some(Action::Scissors),
        _ => None
    }
}

fn to_outcome(outcome: &str) -> Option<Outcome> {
    match outcome {
        "X" => Some(Outcome::Loose),
        "Y" => Some(Outcome::Draw),
        "Z" => Some(Outcome::Win),
        _ => None
    }
}

fn judge(opponent: &Action, you: &Action) -> Outcome {
    match opponent {
        Action::Rock => {
            match you {
                Action::Rock => Outcome::Draw,
                Action::Paper => Outcome::Win,
                Action::Scissors => Outcome::Loose
            }
        },
        Action::Paper => {
            match you {
                Action::Rock => Outcome::Loose,
                Action::Paper => Outcome::Draw,
                Action::Scissors => Outcome::Win
            }
        },
        Action::Scissors => {
            match you {
                Action::Rock => Outcome::Win,
                Action::Paper => Outcome::Loose,
                Action::Scissors => Outcome::Draw
            }
        }
    }
}

fn needed_action(opponent: &Action, expected_outcome: &Outcome) -> Action {
    match opponent {
        Action::Rock => {
            match expected_outcome {
                Outcome::Win => Action::Paper,
                Outcome::Loose => Action::Scissors,
                Outcome::Draw => Action::Rock
            }
        },
        Action::Paper => {
            match expected_outcome {
                Outcome::Win => Action::Scissors,
                Outcome::Loose => Action::Rock,
                Outcome::Draw => Action::Paper
            }
        },
        Action::Scissors => {
            match expected_outcome {
                Outcome::Win => Action::Rock,
                Outcome::Loose => Action::Paper,
                Outcome::Draw => Action::Scissors
            }
        }
    }
}

fn score(outcome: &Outcome, you: &Action) -> i32 {
    let mut score = 0;

    match outcome {
        Outcome::Win => score += 6,
        Outcome::Draw => score += 3,
        Outcome::Loose => score += 0
    }

    match you {
        Action::Rock => score += 1,
        Action::Paper => score += 2,
        Action::Scissors => score += 3
    }

    score
}


fn main() {
    // Read file
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    // Do stuff
    let mut score1 = 0;
    let mut score2 = 0;
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let game: Vec<&str> = line.trim().split(" ").collect();

        if game.len() == 2 {
            let opponent = to_action(game[0]).unwrap();

            // Part 1
            let you = to_action(game[1]).unwrap();
            let outcome = judge(&opponent, &you);
            score1 += score(&outcome, &you);

            // Part 2
            let outcome = to_outcome(game[1]).unwrap();
            let you = needed_action(&opponent, &outcome);
            score2 += score(&outcome, &you);
        }
        else {
            panic!("Bad input {} on line {}", line, index);
        }
    }

    // print result
    println!("Day 2:");
    println!("Answer part one: {}", score1);
    println!("Answer part two: {}", score2);
}