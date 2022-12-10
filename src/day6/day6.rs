use std::collections::HashSet;
use std::io::Result as IoResult;
use crate::helper::file_read::{file_path, read_file};

const EXERCISE_NAME: &str = "day6";

pub fn solve() -> () {
    let string_path = file_path(&EXERCISE_NAME.to_string());
    let lines = read_file(&string_path);

    part1(&lines);
    part2(&lines);
}

fn string_has_duplicates(str: String) -> bool {
    let mut set = HashSet::new();

    for c in str.chars() {
        set.insert(c);
    }

    return set.len() != str.len();
}

fn find_marker(line: &IoResult<String>, marker_size: usize) -> usize {
    let chars = line.as_ref().unwrap().chars().collect::<Vec<char>>();

    let mut from = 0;
    let mut to = marker_size;

    while to <= chars.len() - 1 {
        let slice_str = String::from_iter(&chars[from..to].to_vec());

        if !string_has_duplicates(slice_str) {
            break;
        }

        from = from + 1;
        to = to + 1;
    }

    return to;
}

fn part1(lines: &Vec<IoResult<String>>) -> () {
    assert_eq!(1, lines.len());
    let line = lines.first().unwrap();

    println!("Day 6 - part 1: {}", find_marker(line, 4));
}

fn part2(lines: &Vec<IoResult<String>>) -> () {
    assert_eq!(1, lines.len());
    let line = lines.first().unwrap();

    println!("Day 6 - part 2: {}", find_marker(line, 14));
}
