use std::string::ToString;
use std::io::Result;
use crate::helper::file_read::{file_path, read_file};

const EXERCISE_NAME: &str = "day4";

pub fn solve() -> () {
    let string_path = file_path(&EXERCISE_NAME.to_string());
    let lines = read_file(&string_path);

    part(&lines);
}

fn get_interval_values(line: &String) -> (Vec<i32>, Vec<i32>) {
    let line_fragments = line.split(",").collect::<Vec<&str>>();
    let first_half = line_fragments.first().unwrap()
        .split("-")
        .collect::<Vec<&str>>()
        .iter()
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let second_half = line_fragments.last().unwrap()
        .split("-")
        .collect::<Vec<&str>>()
        .iter()
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    return (first_half, second_half);
}

fn compare(a: &i32, b: &i32, c: &i32, d: &i32) -> bool {
    return a <= b && c >= d;
}

fn part(lines: &Vec<Result<String>>) -> () {
    let mut overlapping_fully = 0;
    let mut overlapping = 0;

    for line in lines {
        let (first_half, second_half) = get_interval_values(line.as_ref().unwrap());
        let (a, b, c, d) =
            (
                first_half.first().unwrap(),
                second_half.first().unwrap(),
                first_half.last().unwrap(),
                second_half.last().unwrap()
            );

        if compare(a, d, c, b) {
            overlapping += 1;
        }

        if compare(a, b, c , d) || compare(b, a, d, c) {
            overlapping_fully += 1;
        }
    }

    println!("Day 4 - part 1: {}", overlapping_fully);
    println!("Day 4 - part 2: {}", overlapping);
}
