use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let mut pairs = Vec::new();
    if let Ok(file) = read_lines("input/day04.txt") {
        for content in file {
            if let Ok(line) = content {
                pairs.push(get_pair(line));
            }
        }
    }
    real_stuff(pairs);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_pair(line : String) -> ((i32, i32, i32), (i32, i32, i32)) {
    let pair : Vec<&str> = line.split(",").collect();
    let mut details = Vec::new();
    for assign in pair {
        details.push( get_assign_details(assign));
    }
    return (details[0], details[1]);
}

fn get_assign_details(assign : &str) -> (i32, i32, i32) {
    let targets : Vec<&str> = assign.split("-").collect();
    let start = targets[0].parse().unwrap();
    let end = targets[1].parse().unwrap();
    let length = end - start + 1;
    return (start, end, length);
}

fn real_stuff(pairs : Vec<((i32, i32, i32),(i32, i32, i32))>) {
    let mut sum = 0;
    let mut sum_partial = 0;
    for pair in pairs {
        //checks for partial overlap
        let gt1 = pair.0.0 >= pair.1.0 && pair.0.0 <= pair.1.1;
        let gt2 = pair.0.1 >= pair.1.0 && pair.0.1 <= pair.1.1;
        if gt1 || gt2{
            sum_partial += 1;
        } else { //check for complete overlap as this is missing
            if pair.0.2 >= pair.1.2 {
                if pair.0.0 <= pair.1.0 && pair.0.1 >= pair.1.1 {
                    sum += 1;
                }
            } else {
                if pair.0.0 >= pair.1.0 && pair.0.1 <= pair.1.1 {
                    sum += 1;
                }
            }
        }
    }
    sum_partial += sum;
    println!("Day 04 Part 1: {}", sum);
    println!("Day 04 Part 2: {}", sum_partial);
}