use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read file
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    // Do stuff
    let mut elf_top1 = 0;
    let mut elf_top2 = 0;
    let mut elf_top3 = 0;
    let mut elf_total = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        match line.parse::<i32>() {
            Ok(n) => elf_total += n,
            Err(_) => {
                if elf_total > elf_top1 {
                    elf_top3 = elf_top2;
                    elf_top2 = elf_top1;
                    elf_top1 = elf_total;
                }
                else if elf_total > elf_top2 {
                    elf_top3 = elf_top2;
                    elf_top2 = elf_total;
                }
                else if elf_total > elf_top3 {
                    elf_top3 = elf_total;
                }

                elf_total = 0;
            }
        }
    }

    // print result
    println!("Day 1:");
    println!("Answer part 1: {}", elf_top1);
    println!("Answer part 2: {}", elf_top1 + elf_top2 + elf_top3);
}
