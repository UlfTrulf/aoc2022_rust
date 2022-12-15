use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path
};
use std::cmp::max;
use std::ffi::c_float;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
enum Interpret {
    #[display("Sensor at x={0}, y={1}: closest beacon is at x={2}, y={3}")]
    Input(i32, i32, i32, i32),

}

fn read_lines<P>(filename: P) -> io::Lines<BufReader<File>>
    where P: AsRef<Path>, {
    let file = File::open(filename).expect("error reading file");
    BufReader::new(file).lines()
}

fn convert_input() -> (Vec<(i32, i32, i32, i32, i32)>, (i32, i32)){
    let mut areas = Vec::new();
    let mut x = (99999, 0);
    let mut y = (99999, 0);
    for content in read_lines("input/day15.txt") {
        let mut point = (99999, 0, 999999,0 , 0);
        if let Ok(line) = content {
            let l = line.parse::<Interpret>().expect("Error i1");
            match l {
                Interpret::Input(sx, sy, bx, by) => {
                    point.0 = sx;
                    point.1 = sy;
                    point.2 = bx;
                    point.3 = by;
                    point.4 = (sx.abs_diff(bx) + sy.abs_diff(by) ) as i32;
                    if sx < x.0 {
                        x.0 = sx;
                    }
                    if bx < x.0 {
                        x.0 = bx;
                    }
                    if sx > x.1 {
                        x.1 = sx;
                    }
                    if bx > x.1 {
                        x.1 = bx;
                    }
                    if sy < y.0 {
                        y.0 = sy;
                    }
                    if by < y.0 {
                        y.0 = by;
                    }
                    if sy > y.1 {
                        y.1 = sy;
                    }
                    if by > y.1 {
                        y.1 = by;
                    }
                }
            }
        }
        areas.push(point);
    }
    return (areas, x)
}

pub fn run() {
    let (p, x) = convert_input();
    part1(p.clone(), x.clone());
    part2(p.clone(), );
}

fn part1(points : Vec<(i32, i32, i32, i32, i32)>, mut xmax : (i32, i32)) {
    let mut sum = 0;
    let y: i32 = 2000000;
    for p in &points {
        if p.0 - p.4 < xmax.0 {
            xmax.0 = p.0 - p.4;
        }
        if p.0 + p.4 > xmax.1 {
            xmax.1 = p.0 + p.4;
        }
    }
    for x in xmax.0..xmax.1 + 1 {
        let mut inrad = false;
        for p in &points {
            if ((x.abs_diff(p.0) + y.abs_diff(p.1)) as i32 <= p.4) & !((x == p.2) & (y == p.3)) {
                inrad = true;
                break;
            }
        }
        if inrad {
            sum += 1;
        }
    }
    println!("Day 15 Part 1: {}", sum);
}

fn part2(points : Vec<(i32, i32, i32, i32, i32)>) {
    let value = 4000000;
    for y in 0.. value + 1 as i32{
        let mut possible = Vec::new();
        for p in &points {
            let diffx = p.4 -  y.abs_diff(p.1) as i32;
            if diffx < 0 {
                continue;
            }
            possible.push((p.0 - diffx, p.0 + diffx));
        }
        possible.sort();
        let mut rays = Vec::new();
        for i in 0..possible.len() {
            if rays.len() == 0 {
                rays.push(possible[i]);
                continue;
            }
            let len = rays.len() - 1;
            if rays[len].1 + 1 >= possible[i].0 {
                if rays[len].1 < possible[i].1 {
                    rays[len].1 = possible[i].1;
                }
                continue;
            }
            rays.push(possible[i])
        }
        let mut x = 0;
        for (low, high) in rays {
            if x > value {
                continue;
            }
            if x < low{
                println!("Day 15 Part 2: {}", (x + 1) as i128 * 4000000 as i128 + y as i128);
            }
            x = max(x, high);
        }
    }
}