use std::collections::{HashMap, VecDeque};
use std::string::ToString;
use std::io::Result as IoResult;
use crate::helper::file_read::{file_path, read_file};

const EXERCISE_NAME: &str = "day5";

pub fn solve() -> () {
    let string_path = file_path(&EXERCISE_NAME.to_string());
    let lines = read_file(&string_path);

    part1(&lines);
    part2(&lines);
}

fn calculate_stack_positions(lines: &Vec<IoResult<String>>, pick_individually: bool) -> String {
    let mut init_stacks = true;
    let mut stacks = HashMap::new();

    for line in lines {
        if line.as_ref().unwrap().is_empty() {
            init_stacks = false;

            continue;
        }

        if init_stacks {
            let chars: Vec<_> = line.as_ref().unwrap().chars().collect();
            let chars_length = chars.len();
            let mut index = 0;

            while index * 4 + 1 <= chars_length - 1 {
                let char = chars[index * 4 + 1];

                if char.is_whitespace() || char.is_numeric() {
                    index = index + 1;
                    continue;
                }

                // Insert char into relevant vector in hashmap
                stacks
                    .entry(index)
                    .or_insert(VecDeque::new())
                    .push_front(char)
                ;

                index = index + 1;
            }

            continue;
        }

        let instructions = line.as_ref().unwrap()
            .split_whitespace()
            .filter_map(|w| w.parse().ok())
            .collect::<Vec<i32>>()
        ;

        assert_eq!(3, instructions.len());

        let amount: usize = instructions[0] as usize;
        let from: usize = (instructions[1] - 1) as usize;
        let to: usize = (instructions[2] - 1) as usize;

        let from_vec = stacks.get(&from).unwrap().to_owned();
        let mut buffer = stacks.get_mut(&from).unwrap().split_off(from_vec.len() - amount);

        if pick_individually {
            // Reverse buffer so we end up with correct order
            buffer = buffer.into_iter().rev().collect();
        }

        stacks.get_mut(&to).unwrap().extend(buffer);
    }

    let mut message = String::new();

    for i in 0..=stacks.len() - 1 {
        message.push(stacks.get(&i).unwrap().to_owned().pop_back().unwrap());
    }

    return message;
}

fn part1(lines: &Vec<IoResult<String>>) -> () {
    println!("Day 5 - part 1: {}", calculate_stack_positions(lines, true));
}

fn part2(lines: &Vec<IoResult<String>>) -> () {
    println!("Day 5 - part 2: {}", calculate_stack_positions(lines, false));
}
