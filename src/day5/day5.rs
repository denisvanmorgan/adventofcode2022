use std::string::ToString;
use std::io::Result;
use crate::helper::file_read::{file_path, read_file};

const EXERCISE_NAME: &str = "day5";

pub fn solve() -> () {
    let string_path = file_path(&EXERCISE_NAME.to_string());
    let lines = read_file(&string_path);

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<Result<String>>) -> () {
    println!("Day 5 - part 1: {}", "empty");
}

fn part2(lines: &Vec<Result<String>>) -> () {
    println!("Day 5 - part 2: {}", "empty");
}
