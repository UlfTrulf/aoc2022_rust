use std::fs;

pub fn run() {
    let contents =
        fs::read_to_string("input/day01.txt")
            .expect("Error reading File!");
    let lines = contents.split("\n");
    let mut current_number = 0;
    let mut elves = vec![];

    for line in lines {
        if line.len() < 2 {
            elves.push(current_number);
            current_number = 0;
        } else {
            let string = &line[0..(line.len() - 1)];
            let number: i32 = string.parse().unwrap_or(0);
            current_number += number;
        }
    }
    elves.sort();
    let capacity = elves[elves.len() - 1] + elves[elves.len() - 2] + elves[elves.len() - 3];
    println!("Day 01 Part 1: {}", elves[elves.len() - 1]);
    println!("Day 01 Part 2: {}", capacity);
}
