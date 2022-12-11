use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path
};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
enum Interpret {
    #[display("Monkey {0}:")]
    Name(i8),
    #[display("  Starting items: {0}")]
    Items(String),
    #[display("  Operation: new = {0}")]
    Op(String),
    #[display("  Test: divisible by {0}")]
    Test(u128),
    #[display("    If true: throw to monkey {0}")]
    T1(i8),
    #[display("    If false: throw to monkey {0}")]
    T2(i8),
    #[display("")]
    Nothing,
}

struct Monkey {
    name: i8,
    items: Vec<u128>,
    operation: String,
    test: u128,
    targets: (usize, usize),
    inspects: u128,
}

impl Monkey {
    pub fn new(name: i8, operation: String, test: u128, target1: i8, target2: i8) -> Self {
        Self {
            name,
            items: Vec::new(),
            operation,
            test,
            targets: (target1 as usize, target2 as usize),
            inspects: 0,
        }
    }
}

pub fn run() {
    let mut input = convert_input();
    part1(input);
    let mut more = convert_input();
    part2(more);

}

fn read_lines<P>(filename: P) -> io::Lines<BufReader<File>>
    where P: AsRef<Path>, {
    let file = File::open(filename).expect("error reading file");
    BufReader::new(file).lines()
}

fn convert_input() -> Vec<Monkey> {
    let mut apes = Vec::new();
    let file = read_lines("input/day11.txt");
    for chunk in file.chunks(7).into_iter().map(|chunk| { chunk.map(|s| s.unwrap())}) {
        let mut name: i8 = 0;
        let mut items: Vec<u128> = Vec::new();
        let mut op = String::new() ;
        let mut test: u128 = 0;
        let mut t1: i8 = 0;
        let mut t2: i8 = 0;
        for line in chunk.map(|l| {l.parse::<Interpret>().expect("Error i1")}) {
            match line {
                Interpret::Name(n) => name = n,
                Interpret::Items(i) => {
                    for k in i.split(", ").collect::<Vec<&str>>(){
                        items.push(k.parse::<u128>().unwrap());
                    }
                },
                Interpret::Op(o) => op = o,
                Interpret::Test(n) => test = n,
                Interpret::T1(t) => t1 = t,
                Interpret::T2(t) => t2 = t,
                _=> print!(""),
            }
        }
        let mut mk = Monkey::new(name, op, test, t1, t2);
        for it in items {
            mk.items.insert(0, it);
        }
        apes.push(mk);

    }
    return apes;
}

fn part1(mut apes: Vec<Monkey>) {
    for level in 0..20 {
        for monkey in 0..apes.len() {
            while apes[monkey].items.len() > 0 {
                let item = apes[monkey].items.pop().unwrap();
                let fear = operation(&apes[monkey].operation,item) / 3;
                let mut target = 0;
                if (fear % apes[monkey].test) == 0 {
                    target = apes[monkey].targets.0;
                } else {
                    target = apes[monkey].targets.1;
                }
                apes[target].items.push(fear);
                apes[monkey].inspects += 1;
            }
        }
    }
    let mut inspectes = Vec::new();
    for mk in apes {
        inspectes.push(mk.inspects);
    }
    inspectes.sort();
    println!("Day 11 Part 1: {}", inspectes[inspectes.len()-1] * inspectes[inspectes.len()-2]);
}

fn operation(op: &str, i: u128) -> u128 {
    let eq = op.split(" ").collect::<Vec<&str>>();
    let mut a = 0;
    if eq[0] == "old" {
        a = i;
    } else {
        a = eq[0].parse().unwrap();
    }
    let mut b = 0;
    if eq[2] == "old" {
        b = i;
    } else {
        b = eq[2].parse().unwrap();
    }
    let mut c = 0;
    match eq[1] {
        "*" => c = a * b,
        "+" => c = a + b,
        "-" => c = a - b,
        "/" => c = a / b,
        _ => println!("cant do Math"),
    }
    return c;
}

fn part2(mut apes: Vec<Monkey>) {
    let mut modi = 1;
    for modifier in &apes {
        modi *= modifier.test;
    }
    for level in 0..10000 {
        for monkey in 0..apes.len() {
            while apes[monkey].items.len() > 0 {
                let item = apes[monkey].items.pop().unwrap();
                let mut fear = operation(&apes[monkey].operation,item);
                let mut target = 0;
                fear %= modi;
                if (fear % apes[monkey].test) == 0 {
                    target = apes[monkey].targets.0;
                } else {
                    target = apes[monkey].targets.1;
                }
                apes[target].items.push(fear);
                apes[monkey].inspects += 1;
            }
        }
    }
    let mut inspectes = Vec::new();
    for mk in apes {
        inspectes.push(mk.inspects);
    }
    inspectes.sort();
    println!("Day 11 Part 2: {}", inspectes[inspectes.len()-1] * inspectes[inspectes.len()-2]);
}