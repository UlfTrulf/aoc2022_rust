use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub fn run() {
    let mut tree = vec![vec![(String::from("/"), 0)]];
    let mut dir = String::new();
    if let Ok(file) = read_lines("input/day07.txt") {
        for content in file {
            if let Ok(line) = content {
                let cmd = line.split(" ").collect::<Vec<&str>>();
                if cmd[0] == "$"  {
                    if cmd[1] == "cd" {
                        dir = get_dir(&dir, cmd[2]);
                        if !contain_dir(&dir, &tree){
                            tree.push(vec![(dir.clone().to_owned(), 0)]);

                        }
                    } else if cmd[1] == "ls" {
                        //idk why i did this this way but it kina works :)
                    } else {
                        println!("command unkown");
                    }
                } else {
                    let dirnumber = find_dir(&dir.clone(), &tree);
                    if dirnumber != tree.len() {
                        let mut size = 0;
                        let mut name = cmd[1].to_owned();
                        if cmd[0] == "dir" {
                            name = dir.clone() + cmd[1] + "/";
                        } else {
                            size = cmd[0].parse().unwrap();
                        }
                        tree[dirnumber].push((name, size))
                    }

                }
            }
        }
    }
    let dirsizes = part1(&tree);
    part2(dirsizes);
}

fn part1 ( tree : &Vec<Vec<(String, i32)>>) -> Vec<(&String, i32)> {
    let mut totalsize = 0;
    let mut dirsizes = Vec::new();
    let mut reversetree = Vec::new();
    for t in tree {
        reversetree.insert(0, t);
    }
    for t in reversetree {
        let mut dirsize = 0;
        for (s, i) in t {
            let mut k = 0;
            if i != &0 {
                k += i;
            }
            else {
                for &(ds, is) in &dirsizes {
                    if ds == s {
                        k += is;
                    }
                }
            }
            dirsize += k;
        }
        if dirsize <= 100000 {
            totalsize += dirsize;
        }
        dirsizes.push((&t[0].0, dirsize));
    }
    println!("Day 07 Part 1: {}", totalsize);
    return dirsizes;
}


fn part2 (dirs : Vec<(&String, i32)>){
    let mut sizes = Vec::new();
    let req = 30000000 - (70000000 - dirs[dirs.len()-1].1);
    for d in dirs {
        if d.1 > req{
            sizes.push(d.1);
        }
    }
    sizes.sort();
    println!("Day 07 Part 2: {}", sizes[0]);

}

fn find_dir(name : &str, tree : &Vec<Vec<(String, i32)>>) -> usize {
    let mut iter = 0;
    while iter < tree.len() {
        if &tree[iter][0].0 == name {
            return iter;
        }
        iter += 1;
    }
    return tree.len()
}

fn contain_dir(dir : &str, tree : &Vec<Vec<(String, i32)>>) -> bool {
    let number = find_dir(dir, tree);
    if number < tree.len(){
        return true;
    }
    return false;
}

fn get_dir (curdir: &str, dest : &str) -> String {
    if dest == ".." {
        let dirvec = curdir.split("/").collect::<Vec<&str>>();
        return dirvec[..dirvec.len() - 2].join("/")[..].to_owned() + "/";
    } else if dest == "/"{
        return "/".to_owned();
    } else {
        return curdir.to_owned() + dest + "/";
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}