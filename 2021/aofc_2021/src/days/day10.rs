use std::{collections::VecDeque, fs};

fn is_opening_char(c: char) -> bool {
    match c {
        '{' | '[' | '(' | '<' => true,
        _ => false,
    }
}

fn get_matching_char(c: char) -> char {
    match c {
        '{' => '}',
        '<' => '>',
        '[' => ']',
        '(' => ')',
        '}' => '{',
        '>' => '<',
        ']' => '[',
        ')' => '(',
        _ => unreachable!(),
    }
}

fn get_error_char_points(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}
fn get_compeltion_char_points(c: char) -> u128 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

pub fn day10_1 (file_name: &str)->  impl crate::AnsType{
	let input_file = format!("{}/aofc_2021/input/{}",env!("ADVENT_OF_CODE_2021"),file_name);
    let _data = fs::read_to_string(input_file).unwrap();
    let lines = _data.split("\n");

    let mut error_points = 0;
    for line in lines {
        let mut opening_chars = VecDeque::<char>::new();
        for c in line.chars() {
            if is_opening_char(c) {
                opening_chars.push_front(c);
            } else {
                if c != get_matching_char(opening_chars.pop_front().unwrap()) {
                    error_points += get_error_char_points(c);
                }
            }
        }
    }
    error_points
}

pub fn day10_2 (file_name: &str)->  impl crate::AnsType{
	let input_file = format!("{}/aofc_2021/input/{}",env!("ADVENT_OF_CODE_2021"),file_name);
    let _data = fs::read_to_string(input_file).unwrap();
    let lines = _data.split("\n");

    let mut did_error;
    let mut scores = vec![];
    for line in lines {
        let mut opening_chars = VecDeque::<char>::new();
        let mut score = 0;
        did_error = false;
        for c in line.chars() {
            if is_opening_char(c) {
                opening_chars.push_front(c);
            } else {
                if c != get_matching_char(opening_chars.pop_front().unwrap()) {
                    did_error = true;
                    break;
                }
            }
        }
        if !did_error {
            while let Some(c) = opening_chars.pop_front() {
                score *= 5;
                score += get_compeltion_char_points(get_matching_char(c));
            }
            scores.push(score);
        }
    }
    scores.sort();
    scores[scores.len() / 2]
}
