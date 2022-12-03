use std::fs;

pub fn run() {
    let contents =
        fs::read_to_string("input/day02.txt")
            .expect("Error reading File!");
    let spl = contents.split("\n");
    let lines : Vec<&str> = spl.collect();
    part1(&lines);
    part2(&lines);
}

fn part1(lines : &Vec<&str>) {
    let mut cumulitve_points = 0;
    let mut points = vec![];

    for line in lines {
        if line.len() > 3 {
            let clean_line = &line[0..(line.len() - 1)]; //whitespace generated during reading
            let result = calc_duel(clean_line);
            cumulitve_points += result;
            points.push(result);
        }
    }
    println!("Day 02 Part 1: {}", cumulitve_points);
}

fn part2(lines : &Vec<&str>) {
    let mut cumulitve_points = 0;
    let mut points = vec![];

    for line in lines {
        if line.len() > 3 {
            let clean_line = &line[0..(line.len() - 1)]; //whitespace generated during reading
            let duel = calc_sign(clean_line);
            let result = calc_duel(&duel);
            cumulitve_points += result;
            points.push(result);
        }
    }
    println!("Day 02 Part 2: {}", cumulitve_points);
}

//somewhere here some enum would've come in handy
fn get_win_sign(input: &str) -> &str{
    let mut sign = "";
    match input {
        "A" => sign = "Y",
        "B" => sign = "Z",
        "C" => sign = "X",
        _ => println!("Error finding sign"),
    }
    return sign;
}

fn get_draw_sign(input: &str) -> &str{
    let mut sign = "";
    match input {
        "A" => sign = "X",
        "B" => sign = "Y",
        "C" => sign = "Z",
        _ => println!("Error finding sign"),
    }
    return sign;
}

fn get_loose_sign(input: &str) -> &str{
    let mut sign = "";
    match input {
        "A" => sign = "Z",
        "B" => sign = "X",
        "C" => sign = "Y",
        _ => println!("Error finding sign"),
    }
    return sign;
}

fn calc_sign(input: &str) -> String{
    let mut sign = "".to_owned();
    match &input[2..3] {
        "X" => sign = format!("{}{}", &input[..2], get_loose_sign(&input[..1])),
        "Y" => sign = format!("{}{}", &input[..2], get_draw_sign(&input[..1])),
        "Z" => sign = format!("{}{}", &input[..2], get_win_sign(&input[..1])),
        _ => println!("Error choosing sign"),
    }
    return sign;
}

fn calc_points(result: &str, sign: &str) -> i32{
    let mut points= 0;
    match result {
        "L" => points += 0,
        "D" => points += 3,
        "W" => points += 6,
        _ => println!("Error calculating Points for Result"),
    }
    match sign {
        "X" => points += 1,
        "Y" => points += 2,
        "Z" => points += 3,
        _ => println!("Error calculating Points for Sign"),
    }
    return points;
}

fn calc_duel(duel: &str) -> i32 {
    let mut result = "";
    match duel {
        "A Z" | "B X" | "C Y" => result = "L",
        "A X" | "B Y" | "C Z" => result = "D",
        "A Y" | "B Z" | "C X" => result = "W",
        _ => println!("Error calculating a duel"),
    }
    let points = calc_points(result, &duel[duel.len()-1..duel.len()]);
    return points;
}