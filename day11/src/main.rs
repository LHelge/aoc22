

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: fn(u64) -> u64,
    divisible: u64,
    if_true: usize,
    if_false: usize,
    inspect_counter: u64
}

// Hardcoded monkey input... probably quicker than parsing it... :D
impl Monkey {
    fn example() -> Vec<Monkey> {
        vec![
            Monkey {items: vec![79, 98], operation: |i| i*19, divisible: 23, if_true: 2, if_false: 3, inspect_counter: 0},
            Monkey {items: vec![54, 65, 75, 74], operation: |i| i+6, divisible: 19, if_true: 2, if_false: 0, inspect_counter: 0},
            Monkey {items: vec![79, 60, 97], operation: |i| i*i, divisible: 13, if_true: 1, if_false: 3, inspect_counter: 0},
            Monkey {items: vec![74], operation: |i| i+3, divisible: 17, if_true: 0, if_false: 1, inspect_counter: 0},
        ]
    }

    fn input() -> Vec<Monkey> {
        vec![
            Monkey {items: vec![59, 74, 65, 86], operation: |i| i*19, divisible: 7, if_true: 6, if_false: 2, inspect_counter: 0},
            Monkey {items: vec![62, 84, 72, 91, 68, 78, 51], operation: |i| i+1, divisible: 2, if_true: 2, if_false: 0, inspect_counter: 0},
            Monkey {items: vec![78, 84, 96], operation: |i| i+8, divisible: 19, if_true: 6, if_false: 5, inspect_counter: 0},
            Monkey {items: vec![97, 86], operation: |i| i*i, divisible: 3, if_true: 1, if_false: 0, inspect_counter: 0},
            Monkey {items: vec![50], operation: |i| i+6, divisible: 13, if_true: 3, if_false: 1, inspect_counter: 0},
            Monkey {items: vec![73, 65, 69, 65, 51], operation: |i| i*17, divisible: 11, if_true: 4, if_false: 7, inspect_counter: 0},
            Monkey {items: vec![69, 82, 97, 93, 82, 84, 58, 63], operation: |i| i+5, divisible: 5, if_true: 5, if_false: 7, inspect_counter: 0},
            Monkey {items: vec![81, 78, 82, 76, 79, 80], operation: |i| i+3, divisible: 17, if_true: 3, if_false: 4, inspect_counter: 0},
        ]
    }
}


fn round(mut monkeys: Vec<Monkey>, relief: bool, magic_number: u64) -> Vec<Monkey> {
    for i in 0..monkeys.len() {
        let monkey = monkeys[i].clone();
        
        for _ in 0..monkey.items.len() {
            monkeys[i].inspect_counter += 1;
            let mut item = monkeys[i].items.pop().unwrap();
            item = (monkey.operation)(item);
            if relief {
                item /= 3;
            }
            else {
                item %= magic_number;
            }
            if item % monkey.divisible == 0 {
                monkeys[monkey.if_true].items.push(item);
            } else {
                monkeys[monkey.if_false].items.push(item);
            }
        }
    }

    monkeys
}


fn part1(mut monkeys: Vec<Monkey>) -> u64 {
    for _ in 0..20 {
        monkeys = round(monkeys, true, 1);
    }
    
    let mut inspected: Vec<u64> = vec![];
    for m in monkeys {
        inspected.push(m.inspect_counter);
    }

    inspected.sort();

    inspected[inspected.len()-1] * inspected[inspected.len()-2]
}


fn part2(mut monkeys: Vec<Monkey>) -> u64 {
    let magic_number: u64 = monkeys.iter().map(|m|m.divisible).product();
    for _ in 0..10000 {
        monkeys = round(monkeys, false, magic_number);
    }
    
    let mut inspected: Vec<u64> = vec![];
    for m in monkeys {
        inspected.push(m.inspect_counter);
    }

    inspected.sort();

    inspected[inspected.len()-1] * inspected[inspected.len()-2]
}

fn main() {
    let monkeys_ex = Monkey::example();
    let monkeys = Monkey::input();

    println!("Part1:");
    println!("Example={}", part1(monkeys_ex));
    println!("Main={}", part1(monkeys));

    let monkeys_ex = Monkey::example();
    let monkeys = Monkey::input();

    println!("Part2:");
    println!("Example={}", part2(monkeys_ex));
    println!("Main={}", part2(monkeys));

}
