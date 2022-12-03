use std::string::ToString;
use std::io::Result;
use crate::helper::file_read::{file_path, read_file};

const EXERCISE_NAME: &str = "day1";

pub fn solve() -> () {
    let string_path = file_path(&EXERCISE_NAME.to_string());
    let lines = read_file(&string_path);

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<Result<String>>) -> () {
    let mut max = 0;
    let mut next = 0;

    for line in lines {
        let is_empty_line = line.as_ref().unwrap().is_empty();

        if is_empty_line {
            if max < next {
                max = next;
            }

            next = 0;

            continue;
        }

        let line_str = line.as_ref().unwrap().to_string();
        next = next + line_str.parse::<i32>().unwrap();
    }

    println!("Day 1 - part 1: {}", max);
}

fn part2(lines: &Vec<Result<String>>) -> () {
    let mut next = 0;
    let mut maxed_elves = [0, 0, 0];

    for line in lines {
        let is_empty_line = line.as_ref().unwrap().is_empty();

        if is_empty_line {
            for n in 0..=2 {
                if maxed_elves[n] < next {
                    let previous_max = maxed_elves[n];
                    maxed_elves[n] = next;

                    if n + 1 <= 2 {
                        maxed_elves[n + 1] = previous_max;
                    }

                    break;
                }
            }

            next = 0;

            continue;
        }

        let line_str = line.as_ref().unwrap().to_string();
        next = next + line_str.parse::<i32>().unwrap();
    }

    println!("Day 1 - part 2: {}", maxed_elves.iter().sum::<i32>());
}
