use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let mut input = convert_input();
    input.reverse();
    part1(input.clone());
    part2(input.clone());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn convert_input() -> Vec<String> {
    let mut orders = Vec::new();
    if let Ok(file) = read_lines("input/day10.txt") {
        for content in file {
            if let Ok(line) = content {
                orders.push(line);
            }
        }
    }
    return orders;
}

fn part1(mut input : Vec<String>) {
    let mut x = 1;
    let mut cycle = 1;
    let mut done = false;
    let mut sum_signal = 0;
    let mut tick = true;
    while !done {
        if (cycle - 20) % 40 == 0 {
            sum_signal += cycle * x;
        }
        if input[input.len() - 1] == "noop" {
            input.pop();
        } else {
            if tick {
                tick = false;
            } else {
                let line = input.pop().unwrap();
                x += line.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
                tick = true;
            }
        }
        cycle += 1;
        if input.len() == 0 {
            done = true;
        }
    }
    println!("Day 10 Part 1: {}", sum_signal);
}

fn part2(mut input : Vec<String>) {
    let mut x : i32 = 1;
    let mut cycle: i32 = 0;
    let mut done = false;
    let mut tick = true;
    let mut crt = Vec::new();
    while !done {
        let pos = cycle % 40;
        let lpos = ((cycle - pos) / 40) as usize;
        if pos == 0 {
            crt.push(String::from(""))
        }
        if pos.abs_diff(x) < 2 {
            crt[lpos].push_str("#");
        } else {
            crt[lpos].push_str(".");
        }
        if input[input.len() - 1] == "noop" {
            input.pop();
        } else {
            if tick {
                tick = false;
            } else {
                let line = input.pop().unwrap();
                x += line.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
                tick = true;
            }
        }
        cycle += 1;
        if input.len() == 0 {
            done = true;
        }
    }
    println!("Day 10 Part 2:");
    for l in crt {
        println!("{l}");
    }
}