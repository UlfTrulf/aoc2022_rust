use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

pub fn run() {
    let mut data1:Vec<Vec<char>> = vec![Vec::new(); 9];
    let mut data2:Vec<Vec<char>> = vec![Vec::new(); 9];
    let mut setup = true;
    let pattern : Regex =  Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    if let Ok(file) = read_lines("input/day05.txt") {
        for content in file {
            if let Ok(line) = content {
                if line.chars().nth(1).unwrap_or_default().is_digit(10) & setup {
                    setup = false;
                }
                if setup {
                    for i in 0..9 {
                        let index = 1 + 4 * i;
                        let char = line.chars().nth(index).unwrap_or_default();
                        if char.is_alphabetic(){
                            data1[i].insert(0, char);
                            data2[i].insert(0, char);
                        }
                    }
                } else {
                    let test = pattern.is_match(&line);
                    if test {
                        let cap = pattern.captures(&line).unwrap();
                        let mut inst = Vec::new();
                        for i in 1..4 {
                            let s : &str = cap.get(i).map_or("", |m| m.as_str());
                            let i : i32 = s.parse().unwrap();
                            inst.push(i as usize);
                        }
                        let mut i = 0;
                        let mut helper = Vec::new();
                        while i < inst[0]{
                            //p1
                            let d1 = data1[inst[1] - 1].pop().unwrap();
                            data1[inst[2] - 1].push(d1);
                            //p2
                            let d2 = data2[inst[1] - 1].pop().unwrap();
                            helper.push(d2);

                            i += 1;
                        }
                        //p2
                        while helper.len() > 0 {
                            data2[inst[2] - 1].push(helper.pop().unwrap());
                        }
                    }
                }

            }
        }
    }
    print!("Day 05 Part 1: ");
    for d in &data1{
        if d.len() > 0 {
            print!("{}", d[d.len() - 1]);
        }
    }
    println!("");
    print!("Day 05 Part 2: ");
    for d in &data2{
        if d.len() > 0 {
            print!("{}", d[d.len() - 1]);
        }
    }
    println!("");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}