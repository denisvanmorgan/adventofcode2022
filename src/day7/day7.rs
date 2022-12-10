use std::collections::{HashMap};
use std::io::Result as IoResult;
use crate::helper::file_read::{file_path, read_file};

const EXERCISE_NAME: &str = "day7";
const CMD: &str = "$";
const DIR_CHANGE: &str = "cd";
const PARENT_DIR: &str = "..";
const DIR: &str = "dir";

pub fn solve() -> () {
    let string_path = file_path(&EXERCISE_NAME.to_string());
    let lines = read_file(&string_path);

    parts(&lines);
}

fn is_command(str_part: &str) -> bool {
    return str_part == CMD;
}

fn is_dir_change(str_part: &str) -> bool {
    return str_part == DIR_CHANGE;
}

fn is_dir(str_part: &str) -> bool {
    return str_part == DIR;
}

fn parts(lines: &Vec<IoResult<String>>) -> () {
    let mut working_dir: Vec<&str> = Vec::new();
    let mut sizes: HashMap<Vec<&str>, usize> = HashMap::new();

    for line in lines {
        let str_parts = line.as_ref().unwrap().split(" ").collect::<Vec<&str>>();

        if is_command(str_parts[0]) {
            if is_dir_change(str_parts[1]) {
                if str_parts[2] == PARENT_DIR {
                    working_dir.pop();
                } else {
                    working_dir.push(&line.as_ref().unwrap()[5..]);
                    sizes.insert(working_dir.clone(), 0);
                }
            }

            continue;
        }

        if is_dir(str_parts[0]) {
            continue;
        }

        for i in (1..working_dir.len() + 1).rev() {
            *sizes.get_mut(&working_dir[0..i]).unwrap()
                += line.as_ref().unwrap().split(" ").next().unwrap().parse::<usize>().unwrap();
        }
    }

    println!("Day 7 - part 1: {}", sizes.values().filter(|&&v| v <= 100000).sum::<usize>());
    println!(
        "Day 7 - part 2: {}",
        sizes
            .values()
            .filter(|&&v| v >= (*sizes.get(&vec!["/"]).unwrap() - 40000000))
            .min()
            .unwrap()
    );
}
