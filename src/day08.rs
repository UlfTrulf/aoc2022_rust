use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let forest = plant_trees();
    part1(&forest);
    part2(&forest);
}

fn part1 (forest : &Vec<Vec<u32>>) {
    let mut counter = 0;
    for x in 0..forest.len(){
        for y in 0..forest[x].len() {
            let v = check_visible_tile((x,y), &forest);
            if v {
                counter += 1
            }
        }
    }
    println!("Day 08 Part 1: {}", counter);
}

fn check_visible_tile(location : (usize, usize), line : &Vec<Vec<u32>>) -> bool {
    let horizon = check_visible(&location.1,&line[location.0]);
    let mut vert = Vec::new();
    for tree in line {
        vert.push(tree[location.1]);
    }
    let vertical = check_visible(&location.0,&vert);
    if horizon | vertical {
        return true;
    }
    return false;
}

fn check_visible(position : &usize, line : &Vec<u32>) -> bool {
    let mut front = true;
    let mut back = true;
    for i in 0..*position {
        if line[i] >= line[position.to_owned()] {
            front = false;
            break;
        }
    }
    for j in (*position + 1)..line.len(){
        if line[j] >= line[position.to_owned()] {
            back = false;
            break;
        }
    }
    if front | back {
        return true;
    }
    return false;
}

fn part2(forest : &Vec<Vec<u32>>){
    let mut current_best = 0;
    for x in 0..forest.len(){
        for y in 0..forest[x].len() {
            let v = rate_scene((x,y), &forest);
            if v > current_best {
                current_best = v;
            }
        }
    }
    println!("Day 08 Part 2: {}", current_best);
}

fn rate_scene(location : (usize, usize), line : &Vec<Vec<u32>>) -> i32 {
    let horizon = count_visible(&location.1,&line[location.0]);
    let mut vert = Vec::new();
    for tree in line {
        vert.push(tree[location.1]);
    }
    let vertical = count_visible(&location.0,&vert);
    let scene = horizon.0 * horizon.1 * vertical.0 * vertical.1;
    return scene;
}

fn count_visible(position : &usize, line : &Vec<u32>) -> (i32, i32) {
    let mut front = 0;
    let mut back = 0;
    for i in 1..(*position + 1) {
        front += 1;
        if line[position.to_owned() - i] >= line[position.to_owned()] {
            break;
        }
    }
    for j in (*position + 1)..line.len() {
        back += 1;
        if line[j] >= line[position.to_owned()] {
            break;
        }
    }
    return (front, back);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn plant_trees() -> Vec<Vec<u32>> {
    let mut forest = Vec::new();
    if let Ok(file) = read_lines("input/day08.txt") {
        for content in file {
            let mut treeline = Vec::new();
            if let Ok(line) = content {
                for tree in line.chars(){
                    treeline.push(tree.to_digit(10).unwrap());
                }
            }
            forest.push(treeline);
        }
    }
    return forest;
}