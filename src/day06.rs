use std::fs;


pub fn run() {
    let content = fs::read_to_string("input/day06.txt")
        .expect("Couldn't read File");
    let input = content.chars();
    let mut searched : Vec<char> = Vec::new();
    let mut p1 = 0;
    let mut p2 = 0;
    for i in input {
        searched.push(i.to_owned());
        if part1(&searched) & (p1 == 0){
            p1 = searched.len()
        }
        if part2(&searched) & (p2 == 0){
            p2 = searched.len()
        }
    }
    println!("Day 06 Part 1: {}", p1);
    println!("Day 06 Part 2: {}", p2);
}

fn part1(searched : &Vec<char>) -> bool {
    return search_last(searched, 4)
}

fn part2(searched : &Vec<char>) -> bool {
    return search_last(searched, 14)
}

fn search_last(searched : &Vec<char>, amount : usize) -> bool {
    if searched.len() > amount - 1 {
        let prev4 = &searched[searched.len() - amount..searched.len()];
        if !has_dup(prev4) {
            return true;
        }
    }
    return false;
}

fn has_dup<T: PartialEq>(slice: &[T]) -> bool {
    for i in 1..slice.len() {
        if slice[i..].contains(&slice[i - 1]) {
            return true;
        }
    }
    false
}