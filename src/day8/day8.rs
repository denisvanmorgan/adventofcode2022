use std::borrow::Borrow;
use std::io::Result as IoResult;
use crate::helper::file_read::{file_path, read_file};

const EXERCISE_NAME: &str = "day8";
const RADIX: u32 = 10;

pub fn solve() -> () {
    let string_path = file_path(&EXERCISE_NAME.to_string());
    let lines = read_file(&string_path);

    part1(&lines);
    part2(&lines);
}

fn compare_forest_parts(tree: &usize, first_part_max: &usize, second_part_max: &usize) -> bool {
    return first_part_max >= tree && second_part_max >= tree;
}

fn get_forest_parts(vec: &Vec<usize>, split_at: usize) -> (Vec<&usize>, Vec<&usize>) {
    let (first, second) = vec.split_at(split_at);
    let mut first = Vec::from_iter(first);
    first.pop();

    return (first, Vec::from_iter(second));
}

fn get_col_values(matrix: &Vec<Vec<usize>>, x: usize) -> Vec<usize> {
    let mut col = Vec::new();

    for i in 0..=matrix.len() - 1 {
        col.push(matrix[i][x]);
    }

    return col;
}

fn is_visible(matrix: &Vec<Vec<usize>>, x: usize, y: usize) -> bool {
    let tree = &matrix[y][x];
    let row = &matrix[y];
    let col = get_col_values(matrix, x);

    let mut visible_in_row = true;
    let mut visible_in_col = true;

    // Row check
    let (first_row_part, second_row_part) = get_forest_parts(row, x + 1);

    if compare_forest_parts(
        tree,
        first_row_part.iter().max().unwrap(),
        second_row_part.iter().max().unwrap()
    ) {
        visible_in_row = false;
    }

    if visible_in_row {
        return true;
    }

    // Col check
    let (first_col_part, second_col_part) = get_forest_parts(&col, y + 1);

    if compare_forest_parts(
        tree,
        first_col_part.iter().max().unwrap(),
        second_col_part.iter().max().unwrap()
    ) {
        visible_in_col = false;
    }

    if visible_in_col {
        return true;
    }

    return false;
}

fn part1(lines: &Vec<IoResult<String>>) -> () {
    let mut matrix = Vec::new();

    for line in lines {
        let chars = line
            .as_ref()
            .unwrap()
            .chars()
            .map(|c| c.to_digit(RADIX).unwrap() as usize)
            .collect::<Vec<usize>>()
        ;
        matrix.push(chars);
    }

    let row_len = matrix[0].len();
    let col_len = matrix.len();
    let mut visible = 0;
    // Visible trees on both sides of rows
    visible = visible + col_len * 2;
    // Visible trees on top and bottom of matrix (-2 because we count them in rows)
    visible = visible + ((row_len - 2) * 2);

    let starting_x_index = 1;
    let starting_y_index = 1;

    for i in starting_x_index..=col_len - 2 {
        for j in starting_y_index..=row_len - 2 {
            if is_visible(&matrix, j, i) {
                visible = visible + 1;
            }
        }
    }

    println!("Day 8 - part 1: {}", visible);
}

fn calculate_score_for_forest_row(row: &Vec<&&usize>, tree: &usize) -> usize {
    let mut counter = 0;

    if row.len() == 0 {
        return 0;
    }

    for i in row.iter() {
        counter = counter + 1;
        // TODO another pointer hell...
        if i >= &&&tree {
            break;
        }
    }

    return counter;
}

fn get_tree_score(matrix: &Vec<Vec<usize>>, x: usize, y: usize) -> usize {
    let tree = &matrix[y][x];
    let row = &matrix[y];
    let col = get_col_values(matrix, x);
    let mut score = 0;

    let (first_row_part, second_row_part) = get_forest_parts(row, x + 1);
    let (first_col_part, second_col_part) = get_forest_parts(&col, y + 1);
    let mod_first_row_part = first_row_part.iter().rev().collect::<Vec<&&usize>>();
    let mod_first_col_part = first_col_part.iter().rev().collect::<Vec<&&usize>>();
    // TODO try to remove this dumb pointer hell
    let data = vec![mod_first_col_part, second_col_part.iter().collect::<Vec<&&usize>>(), mod_first_row_part, second_row_part.iter().collect::<Vec<&&usize>>()];

    for vec in data.iter() {
        let forest_score = calculate_score_for_forest_row(vec, tree);

        if score == 0 {
            score = forest_score;
        } else {
            score = score * forest_score;
        }
    }

    return score;
}

fn part2(lines: &Vec<IoResult<String>>) -> () {
    let mut matrix = Vec::new();

    for line in lines {
        let chars = line
            .as_ref()
            .unwrap()
            .chars()
            .map(|c| c.to_digit(RADIX).unwrap() as usize)
            .collect::<Vec<usize>>()
            ;
        matrix.push(chars);
    }

    let row_len = matrix[0].len();
    let col_len = matrix.len();
    let mut score = 0;

    for i in 1..=col_len - 2 {
        for j in 1..=row_len - 2 {
            let tree_score = get_tree_score(&matrix, j, i);

            if score < tree_score {
                score = tree_score;
            }
        }
    }

    println!("Day 8 - part 2: {}", score);
}
