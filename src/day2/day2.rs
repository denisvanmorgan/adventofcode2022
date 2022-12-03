use std::collections::HashMap;
use std::string::ToString;
use std::io::Result;
use crate::helper::file_read::{file_path, read_file};

const EXERCISE_NAME: &str = "day2";
const ASCII_VALUE_OFFSET: u8 = 23;
const ASCII_MIN_CHAR: u8 = 65;
const ASCII_MAX_CHAR: u8 = 67;

pub fn solve() -> () {
    let string_path = file_path(&EXERCISE_NAME.to_string());
    let lines = read_file(&string_path);

    part1(&lines);
    part2(&lines);
}

fn split_moves(line: &Result<String>) -> Vec<&str> {
    return line.as_ref().unwrap().split(" ").collect();
}

fn get_value_map() -> HashMap<u8, i32> {
    let mut value_map = HashMap::new();

    // ASCII values of A, B, C
    // We can figure out X, Y, Z values if we substract the ASCII_VALUE_OFFSET from our move
    value_map.insert(65, 1); // Bit representation 01 - Rock
    value_map.insert(66, 2); // Bit representation 10 - Paper
    value_map.insert(67, 3); // Bit representation 11 - Scissors

    return value_map;
}

fn calculate_binary_diff(opponent_move: &u8, our_move: &u8) -> i32 {
    let value_map = get_value_map();

    // For our opponent, we prepend binary 1 and for us we prepend binary 0
    // In this case, all possible decimal results for opponent are 4,5,6 and for us 0,1,2
    // That means if we calculate "result mod 3" and get 0 we win
    // if the result from modulo is 4, it's draw
    // if the result from modulo is 2, opponent wins
    return (value_map.get(&opponent_move).unwrap() | 1 << (2))
        - (value_map.get(&our_move).unwrap() | 0 << (2));
}

fn part1(lines: &Vec<Result<String>>) -> () {
    let mut score: i32 = 0;
    let value_map = get_value_map();

    for line in lines {
        let split_moves = split_moves(line);
        let opponent_move: u8 = split_moves[0].as_bytes()[0];
        let our_move: u8 = split_moves[1].as_bytes()[0] - ASCII_VALUE_OFFSET;

        // Draw
        if opponent_move == our_move {
            score += 3;
            score += value_map.get(&our_move).unwrap();

            continue;
        }

        // Opponent win
        if calculate_binary_diff(&opponent_move, &our_move) % 3 != 0 {
            score += value_map.get(&our_move).unwrap();

            continue;
        }

        // We win!
        score += 6;
        score += value_map.get(&our_move).unwrap();
    }

    println!("Day 2 - part 1: {}", score);
}

fn get_winning_move(opponent_move: &u8) -> u8 {
    if opponent_move + 1 > ASCII_MAX_CHAR {
        return ASCII_MIN_CHAR;
    }

    return opponent_move + 1 as u8;
}

fn get_loosing_move(opponent_move: &u8) -> u8 {
    if opponent_move - 1 < ASCII_MIN_CHAR {
        return ASCII_MAX_CHAR;
    }

    return opponent_move - 1 as u8;
}

fn part2(lines: &Vec<Result<String>>) -> () {
    let mut score: i32 = 0;
    let value_map = get_value_map();

    for line in lines {
        let split_moves = split_moves(line);
        let opponent_move: u8 = split_moves[0].as_bytes()[0];
        let our_move: u8 = split_moves[1].as_bytes()[0] - ASCII_VALUE_OFFSET;

        if value_map.get(&our_move).unwrap().to_owned() == 1 as i32 {
            score += value_map.get(&get_loosing_move(&opponent_move)).unwrap();

            continue;
        }

        if value_map.get(&our_move).unwrap().to_owned() == 2 as i32 {
            score += 3;
            score += value_map.get(&opponent_move).unwrap();

            continue;
        }

        score += 6;
        score += value_map.get(&get_winning_move(&opponent_move)).unwrap();
    }

    println!("Day 2 - part 2: {}", score);
}
