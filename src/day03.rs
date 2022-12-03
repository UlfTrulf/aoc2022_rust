use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    part1();
    part2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(){
    let mut sum_of_prio = 0;
    if let Ok(file) = read_lines("input/day03.txt") {
        for content in file {
            if let Ok(line) = content {
                let comp1 = &line[..(line.len() / 2)];
                let comp2 = &line[(line.len() / 2)..];
                let sim = get_similar(comp1, comp2);
                let mut prio = 0;
                for c in sim {
                     prio += calc_prio(c);
                }
                sum_of_prio += prio;


            }
        }
    }
    println!("Day 03 Part 1: {}", sum_of_prio);
}

fn part2(){
    let mut sum_of_prio = 0;
    if let Ok(file) = read_lines("input/day03.txt") {
        let mut group = Vec::new();
        for content in file {
            if let Ok(line) = content {
                group.push(line);
                if group.len() == 3 {
                    let sim = get_group_sim(group);
                    let mut prio = 0;
                    for c in sim {
                        prio += calc_prio(c);
                    }
                    sum_of_prio += prio;
                    group = Vec::new();
                }
            }
        }
    }
    println!("Day 03 Part 2: {}", sum_of_prio);
}

fn get_similar(string1 : &str, string2 : &str) -> Vec<char>{
    let mut similar = Vec::new();
    for c in string1.chars(){
        for d in string2.chars(){
            if c == d && !similar.contains(&c){
                similar.push(c);
            }
        }
    }
    return similar;
}

fn calc_prio(item : char) -> i32 {
    let mut prio = item as i32;
    if prio > 96 {
        prio -= 96;
    } else {
        prio -= 38;
    }
    return prio;
}

fn char_to_string(vec : Vec<char>) -> String {
    let mut s = String::new();
    for c in vec {
        s.push(c);
    }
    return s;
}

fn get_group_sim(group : Vec<String>) -> Vec<char>{
    let mut iterator = group.len() - 1;
    let mut sim = get_similar(group[iterator].as_str(), group[(iterator-1)].as_str());
    while iterator > 0 {
        let current_common = char_to_string(sim);
        let new_sim = get_similar(group[iterator].as_str(), group[(iterator-1)].as_str());
        let new_common = char_to_string(new_sim);
        sim = get_similar(new_common.as_str(), current_common.as_str());
        iterator -= 1;
    }
    return sim;
}