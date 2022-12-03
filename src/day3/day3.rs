use std::collections::{HashMap, HashSet};
use std::string::ToString;
use std::io::Result;
use crate::helper::file_read::{file_path, read_file};

const EXERCISE_NAME: &str = "day3";
const UPPERCASE_ASCII_VALUE_OFFSET: u8 = 38;
const LOWERCASE_ASCII_VALUE_OFFSET: u8 = 96;

pub fn solve() -> () {
    let string_path = file_path(&EXERCISE_NAME.to_string());
    let lines = read_file(&string_path);

    part1(&lines);
    part2(&lines);
}

fn get_offset_for_char(char: &char) -> u8 {
    if char.is_ascii_uppercase() {
        return UPPERCASE_ASCII_VALUE_OFFSET;
    }

    return LOWERCASE_ASCII_VALUE_OFFSET;
}

fn get_sum_for_char(char: &char) -> u32 {
    return (char.to_string().as_bytes().first().unwrap() - get_offset_for_char(&char)) as u32;
}

fn get_sum_from_chars(duplicate_chars: &Vec<char>) -> u32 {
    let mut sum: u32 = 0;

    for duplicate_char in duplicate_chars {
        sum += (duplicate_char.to_string().as_bytes().first().unwrap()
            - get_offset_for_char(&duplicate_char)) as u32;
    }

    return sum;
}

fn part1(lines: &Vec<Result<String>>) -> () {
    let mut duplicates = String::new();

    for line in lines {
        let line_chars: Vec<char> = line.as_ref().unwrap().chars().collect();
        let (first_half, second_half) = line_chars.split_at(line_chars.len() / 2);

        let mut seen = String::new();

        for char in first_half {
            if second_half.contains(char) && !seen.find(*char).is_some() {
                seen.push(char.to_owned());
                duplicates.push(char.to_owned());
            }
        }

        seen.clear();
    }

    let duplicate_chars: Vec<char> = duplicates.chars().collect();

    println!("Day 3 - part 1: {}", get_sum_from_chars(&duplicate_chars));
}

fn remove_duplicates_from_string(string: &String) -> String {
    let mut seen = HashSet::new();
    let mut result = String::new();

    for c in string.chars() {
        if !seen.contains(&c) {
            seen.insert(c);
            result.push(c);
        }
    }

    return result;
}

fn part2(lines: &Vec<Result<String>>) -> () {
    let mut char_counts = HashMap::new();
    let mut buffer = String::new();
    let mut check = 1;
    let mut sum = 0;

    for line in lines {
        buffer.push_str(&*remove_duplicates_from_string(line.as_ref().unwrap()));

        if check < 3 {
            check += 1;
            continue;
        }

        for ch in buffer.chars() {
            *char_counts.entry(ch).or_insert(0) += 1;
        }

        for (char, count) in char_counts.to_owned() {
            if count == 3 {
                sum += get_sum_for_char(&char);
            }
        }

        buffer.clear();
        char_counts.clear();
        check = 1;
    }

    println!("Day 3 - part 2: {}", sum);
}
