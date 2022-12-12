use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path
};

pub fn run() {
    let mut input = convert_input();
    println!("Day 12 Part 1: {}", get_distance(&input.0, input.1, input.2));
    let mut best_distance = 999;
    for y in 0..input.0.len() {
        for x in 0..input.0[y].len() {
           if input.0[y][x] as i32 == 97 {
               let dst = get_distance(&input.0, (x as i32,y as i32), input.2);
               if best_distance > dst {
                   best_distance = dst;
               }
           }
        }
    }
    println!("Day 12 Part 2: {}", best_distance);
}

fn read_lines<P>(filename: P) -> io::Lines<BufReader<File>>
    where P: AsRef<Path>, {
    let file = File::open(filename).expect("error reading file");
    BufReader::new(file).lines()
}

fn convert_input() -> (Vec<Vec<char>>, (i32,i32), (i32,i32))  {
    let mut grid = Vec::new();
    let mut s = (0,0);
    let mut e = (0,0);
    let mut file = Vec::new();
    for line in read_lines("input/day12.txt") {
        if let Ok(content) = line {
            file.push(content);
        }
    }
    for y in 0..file.len(){
        let chars = file[y].chars().collect::<Vec<char>>();
        let mut horizon = Vec::new();
        for x in 0..chars.len() {
            match chars[x].to_string().as_str() {
                "S" => {
                    horizon.push("a".to_string().chars().next().unwrap());
                    s.0 = x as i32;
                    s.1 = y as i32;
                },
                "E" => {
                    horizon.push("z".to_string().chars().next().unwrap());
                    e.0 = x as i32;
                    e.1 = y as i32;
                },
                _ => horizon.push(chars[x]),
            }
        }
        grid.push(horizon);

    }
    return (grid, s, e)
}

fn get_distance(grid: &Vec<Vec<char>>, start: (i32, i32), end: (i32, i32)) -> i32{
    let mut inspect = vec![(0, start.0, start.1)];
    let mut visited = vec![(start.0, start.1)];
    let mut searching = true;
    while searching {
        if inspect.len() == 0 {
            break;
        }
        let position = inspect.pop().unwrap();
        for (a,b) in [(position.1, position.2 + 1), (position.1 + 1, position.2),
                    (position.1 -1, position.2), (position.1, position.2 - 1)] {
            if (a < 0) | (b < 0 ) | (a >= grid[0].len() as i32) | (b >= grid.len() as i32) {
                continue;
            }
            let x = a as usize;
            let y = b as usize;
            let values = ((grid[y][x] as i32), (grid[position.2 as usize][position.1 as usize] as i32));
            if visited.contains(&(a,b)) | (values.0 - values.1 > 1){
                continue;
            }
            if (a, b) == end {
                return position.0 + 1;
            }
            visited.push((a, b));
            inspect.insert(0, (position.0 + 1, a, b));
        }
    }
    return 999;
}
