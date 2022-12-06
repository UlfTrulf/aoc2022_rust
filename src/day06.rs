use std::fs;


pub fn run() {
    let content = fs::read_to_string("input/day06.txt")
        .expect("Couldn't read File");
    part1(&content);
    part2(&content);

}

fn part1(content : &String) {
    let input = content.chars();
    let mut searched : Vec<char> = Vec::new();
    for i in input {
        searched.push(i.to_owned());
        if searched.len() > 3 {
            let prev4 = &searched[searched.len() - 4..searched.len()];
            if !has_dup(prev4) {
                break;
            }
        }
    }
    println!("Day 06 Part 1: {}", searched.len())
}

fn part2(content : &String) {
    let input = content.chars();
    let mut searched : Vec<char> = Vec::new();
    for i in input {
        searched.push(i.to_owned());
        if searched.len() > 13 {
            let prev4 = &searched[searched.len() - 14..searched.len()];
            if !has_dup(prev4) {
                break;
            }
        }
    }
    println!("Day 06 Part 2: {}", searched.len())
}

fn has_dup<T: PartialEq>(slice: &[T]) -> bool {
    for i in 1..slice.len() {
        if slice[i..].contains(&slice[i - 1]) {
            return true;
        }
    }
    false
}