use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path
};
use itertools::Itertools;

pub fn run() {
    stuff();
}

fn read_lines<P>(filename: P) -> io::Lines<BufReader<File>>
    where P: AsRef<Path>, {
    let file = File::open(filename).expect("error reading file");
    BufReader::new(file).lines()
}

fn stuff() {
    let mut pairs = Vec::new();
    let mut sum = 0;
    let mut c = 1;
    let file = read_lines("input/day13.txt");
    for chunk in file.chunks(3).into_iter().map(|chunk| { chunk.map(|s| s.unwrap()) }) {
        let mut sides = Vec::new();
        for line in chunk{
            let mut l = line.chars().collect::<Vec<char>>();
            l.pop();
            l.reverse();
            l.pop();
            sides.push(l)
        }
        let ordered = check_ordered(sides[0].clone(), sides[1].clone());
        if ordered == 1 {
            pairs.push(sides[0].clone());
            pairs.push(sides[1].clone());
            sum += c;
        } else {
            pairs.push(sides[1].clone());
            pairs.push(sides[0].clone());
        }
        c += 1;
    }
    println!("Day 13 Part 1: {}", sum);
    let mut decoder1 = "[[2]]".chars().collect::<Vec<char>>();
    let mut decoder2 = "[[6]]".chars().collect::<Vec<char>>();
    decoder1.reverse();
    decoder2.reverse();
    pairs.push(decoder1.clone());
    pairs.push(decoder2.clone());
    for i in 0..pairs.len() {
        for j in 0..pairs.len() - i - 1 {
            if check_ordered(pairs[j + 1].clone(), pairs[j].clone()) != 1 {
                pairs.swap(j, j + 1);
            }
        }
    }
    let mut di = (0,0);
    for p in 0..pairs.len() {
        if pairs[p] == decoder1 {
            di.0 = p;
        } else if pairs[p] == decoder2 {
            di.1 = p;
        }
    }
    println!("Day 13 Part 2: {}", di.0 * di.1);
}

fn check_ordered (mut left : Vec<char>, mut right: Vec<char>) -> i8 {
    let lbr = "[".chars().next().unwrap();
    let rbr = "]".chars().next().unwrap();
    let kom = ",".chars().next().unwrap();
    while right.len() > 0 {
        if left.len() == 0 {
            return 1;
        }
        let mut l = left.pop().unwrap();
        let mut r = right.pop().unwrap();
        if (l == lbr) | (r == lbr) {
            let mut lvec = Vec::new();
            let mut rvec = Vec::new();
            if l == lbr {
                l = left.pop().unwrap();
                if l != rbr {
                    let mut count = 1;
                    while count > 0{
                        if l == lbr {
                            count += 1;
                        }
                        lvec.push(l);
                        l = left.pop().unwrap();
                        if l == rbr {
                            count -= 1;
                        }
                    }
                }

            } else {
                lvec = vec![l];
                let mut dl = l.to_string().parse::<i32>().unwrap();
                if (dl == 1) & (left.len() > 0){
                    if left[left.len() - 1] == "0".chars().next().unwrap() {
                        lvec.push(left.pop().unwrap());
                    }
                }
            }
            if r == lbr {
                r = right.pop().unwrap();
                if r != rbr {
                    let mut count = 1;
                    while count > 0{
                        if r == lbr {
                            count += 1;
                        }
                        rvec.push(r);
                        r = right.pop().unwrap();
                        if r == rbr {
                            count -= 1;
                        }
                    }
                }
            } else {
                rvec = vec![r];
                let mut dr = r.to_string().parse::<i32>().unwrap();
                if (dr == 1) & (right.len() > 0){
                    if right[right.len() - 1] == "0".chars().next().unwrap() {
                        rvec.push(right.pop().unwrap());
                    }
                }
            }
            lvec.reverse();
            rvec.reverse();
            let check = check_ordered(lvec, rvec);
            if check != 0 {
                return check;
            }
            continue;
        }
        if (l == kom) & (r == kom) {
            continue;
        }
        let mut dl = l.to_string().parse::<i32>().unwrap();
        let mut dr = r.to_string().parse::<i32>().unwrap();
        if (dl == 1) & (left.len() > 0){
            if left[left.len() - 1] == "0".chars().next().unwrap() {
                left.pop();
                dl = 10;
            }
        }
        if (dr == 1) & (right.len() > 0){
            if right[right.len() - 1] == "0".chars().next().unwrap() {
                right.pop();
                dr = 10;
            }
        }
        if dl > dr {
            return 2
        } else if dr > dl {
            return 1
        }
    }
    if left.len() > 0 {
        return 2;
    }
    return 0;
}