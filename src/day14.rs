use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path
};

fn read_lines<P>(filename: P) -> io::Lines<BufReader<File>>
    where P: AsRef<Path>, {
    let file = File::open(filename).expect("error reading file");
    BufReader::new(file).lines()
}

fn convert_input() -> (Vec<Vec<i8>>, (i32,i32))  {
    let mut grid = Vec::new();
    let mut x = (1000,0);
    let mut y = (0,0);
    let mut rocks = Vec::new();
    for line in read_lines("input/day14.txt") {
        let mut formation = Vec::new();
        if let Ok(content) = line {
            let corners = content.split(" -> ").collect::<Vec<&str>>();
            for c in corners {
                let tupel = c.split(",").collect::<Vec<&str>>();
                let tu = (tupel[0].parse::<i32>().unwrap(),
                              tupel[1].parse::<i32>().unwrap());
                if tu.0 < x.0 {
                    x.0 = tu.0;
                }
                if tu.0 > x.1 {
                    x.1 = tu.0;
                }
                if tu.1 < y.0 {
                    y.0 = tu.1;
                }
                if tu.1 > y.1 {
                    y.1 = tu.1;
                }
                formation.push(tu);
            }
        }
        rocks.push(formation);
    }

    for _a in 0..(x.1 - x.0 + 1) {
        let mut column = Vec::new();
        for _b in y.0..(y.1 + 1) {
            column.push(0);
        }
        grid.push(column)
    }
    for mut formation in rocks.clone() {
        for position in 0 .. formation.len() - 1 {
            let mut now = formation[position];
            let next = formation[position + 1];
            grid[(now.0 - x.0) as usize][now.1 as usize] = 2;
            while now.0 < next.0 {
                grid[(now.0 - x.0) as usize][now.1 as usize] = 2;
                now.0 += 1;
            }
            while now.0 > next.0 {
                grid[(now.0 - x.0) as usize][now.1 as usize] = 2;
                now.0 -= 1;
            }
            while now.1 < next.1 {
                grid[(now.0 - x.0) as usize][now.1 as usize] = 2;
                now.1 += 1;
            }
            while now.1 > next.1 {
                grid[(now.0 - x.0) as usize][now.1 as usize] = 2;
                now.1 -= 1;
            }
        }
        grid[(formation[formation.len()-1].0 - x.0) as usize][formation[formation.len()-1].1 as usize] = 2;

    }
    grid[(500 - x.0) as usize][0] = 1;
    return (grid, x)
}

pub fn run() {
    let (cave, x) = convert_input();
    part1(cave.clone(), 500 - x.0);
    part2(cave.clone(), 500 - x.0);
}

fn part1(mut cave : Vec<Vec<i8>>, source : i32) {
    let mut count = 0;
    let mut inbounds = true;
    while inbounds {
        let mut x = source as usize;
        let mut y = 0;
        let mut falling = true;
        let mut wleft = true;
        let mut wright = true;
        let mut steps = 0;
        while falling {
            if cave[x][y + 1] == 0 {
                y += 1;
                steps += 1;
            } else if !wleft {
                falling = false;
                inbounds = false;
            } else if wleft & (cave[x - 1][y +1] == 0) {
                x -= 1;
                y += 1;
                steps += 1;
            } else if !wright {
                falling = false;
                inbounds = false;
            } else if wright & (cave[x + 1][y +1] == 0) {
                x += 1;
                y += 1;
                steps += 1;
            } else {
                cave[x][y] = 3;
                count += 1;
                falling = false;
            }
            if x <= 0 {
                wleft = false;
            }
            if x >= cave.len() - 1 {
                wright = false;
            }
            if y >= cave[0].len() - 1 {
                falling = false;
                inbounds = false;
            }
        }
        if steps == 0 {
            inbounds = false;
        }
    }
    println!("Day 14 Part 1: {}", count);
}

fn part2(mut cave : Vec<Vec<i8>>, source : i32) {
    let mut side = Vec::new();
    for _i in 0.. cave[0].len() + 1 {
        side.push(0);
    }
    side.push(2);
    for mut c in 0..cave.len() {
        cave[c].push(0);
        cave[c].push(2);
    }

    let mut count = 0;
    let mut inbounds = true;
    let mut offset = 0;
    while inbounds {
        let mut x = offset + source as usize;
        let mut y = 0;
        let mut falling = true;
        let mut steps = 0;
        while falling {
            if cave[x][y + 1] == 0 {
                y += 1;
                steps += 1;
            } else if cave[x - 1][y +1] == 0 {
                x -= 1;
                y += 1;
                steps += 1;
            } else if cave[x + 1][y +1] == 0 {
                x += 1;
                y += 1;
                steps += 1;
            } else {
                cave[x][y] = 3;
                count += 1;
                falling = false;
                if (x == source as usize) & (y == 0){
                    inbounds = false;
                }
            }
            if x <= 0 {
                cave.insert(0, side.clone());
                x += 1;
                offset += 1;
            }
            if x >= cave.len() - 3 {
                cave.push(side.clone());
            }
        }
        if steps == 0 {
            inbounds = false;
        }
    }
    println!("Day 14 Part 2: {}", count);
}