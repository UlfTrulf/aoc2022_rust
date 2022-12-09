use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let input = convert_input();
    part1(&input);
    part2(&input);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn convert_input() -> Vec<(String, i32)> {
    let mut orders = Vec::new();
    if let Ok(file) = read_lines("input/day09.txt") {
        for content in file {
            if let Ok(line) = content {
                let cmd = line.split(" ").collect::<Vec<&str>>();
                orders.push((cmd[0].to_owned(), cmd[1].parse::<i32>().unwrap()));
            }
        }
    }
    return orders;
}

fn part1(input : &Vec<(String, i32)>){
    let mut head = (0,0);
    let mut tailpos = vec![(0,0)];
    for order in input.to_owned() {
        let dir = order.0.as_str();
        for _i in 0..order.1 {
            head = head_movement(dir, head);
            let t = tail_movement(&head, &tailpos[tailpos.len()-1]);
            tailpos.push(t);
        }
    }
    tailpos.sort();
    tailpos.dedup();
    println!("Day 09 Part 1: {}", tailpos.len());
}

fn head_movement(dir : &str, mut head : (i32, i32)) -> (i32, i32) {
    match dir{
        "U"=>head.1 += 1,
        "D"=>head.1 -= 1,
        "R"=>head.0 += 1,
        "L"=>head.0 -= 1,
        _=>println!("Invalid Direction"),
    }
    return head;
}

fn tail_movement(head : &(i32, i32), tail : &(i32, i32) )  -> (i32,i32) {
    let x = head.0 - tail.0;
    let y = head.1 - tail.1;
    let mut newt = tail.to_owned();
    if x.abs() + y.abs() <= 2{
        match x {
            2 => newt.0 += 1,
            -2 => newt.0 += -1,
            _=> newt.0 += 0,
        }
        match y {
            2 => newt.1 += 1,
            -2 => newt.1 += -1,
            _=> newt.1 += 0,
        }
    } else {
        match x {
            2 | 1 => newt.0 += 1,
            -2 | -1 => newt.0 += -1,
            _=> newt.0 = 0,
        }
        match y {
            2 | 1 => newt.1 += 1,
            -2 | -1  => newt.1 += -1,
            _=> newt.1 += 0,
        }
    }
    return newt;
}

fn part2(input : &Vec<(String, i32)>){
    let mut head = (0,0);
    let mut pos = vec![vec![(0,0)]];
    for _p in 0..8 {
        pos.push(vec![(0,0)]);
    }
    for order in input.to_owned() {
        let dir = order.0.as_str();
        for _i in 0..order.1 {
            head = head_movement(dir, head);
            let t = tail_movement(&head, &pos[0][pos[0].len()-1]);
            pos[0].push(t);
            for p in 1..9 {
                let t = tail_movement(&pos[p-1][pos[p-1].len()-1], &pos[p][pos[p].len()-1]);
                pos[p].push(t);
            }
        }
    }
    let len  = pos.len() - 1;
    pos[len].sort();
    pos[len].dedup();
    println!("Day 09 Part 2: {}", pos[len].len());
}