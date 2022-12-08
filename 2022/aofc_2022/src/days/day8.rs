use std::{collections::HashSet, fs};

pub fn day8_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let trees = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|n| n as i32))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut visable_trees: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..trees.len() {
        let mut tallest = -1;
        for x in 0..trees[y].len() {
            if trees[y][x] > tallest {
                tallest = trees[y][x];
                visable_trees.insert((y, x));
            }
        }
    }
    for x in 0..trees[0].len() {
        let mut tallest = -1;
        for y in 0..trees.len() {
            if trees[y][x] > tallest {
                tallest = trees[y][x];
                visable_trees.insert((y, x));
            }
        }
    }
    for y in 0..trees.len() {
        let mut tallest = -1;
        for x in (0..trees[y].len()).rev() {
            if trees[y][x] > tallest {
                tallest = trees[y][x];
                visable_trees.insert((y, x));
            }
        }
    }
    for x in 0..trees[0].len() {
        let mut tallest = -1;
        for y in (0..trees.len()).rev() {
            if trees[y][x] > tallest {
                tallest = trees[y][x];
                visable_trees.insert((y, x));
            }
        }
    }

    visable_trees.len()
}

fn calc_scenic_score(trees: &Vec<Vec<i32>>, tree_x: usize, tree_y: usize) -> i32 {
    let mut score = Vec::new();

    score.push(0);
    for y in (0..tree_y).rev() {
        *score.last_mut().unwrap() += 1;
        if trees[y][tree_x] >= trees[tree_y][tree_x] {
            break;
        }
    }
    score.push(0);
    for y in (tree_y + 1)..trees.len() {
        *score.last_mut().unwrap() += 1;
        if trees[y][tree_x] >= trees[tree_y][tree_x] {
            break;
        }
    }

    score.push(0);
    for x in (0..tree_x).rev() {
        *score.last_mut().unwrap() += 1;
        if trees[tree_y][x] >= trees[tree_y][tree_x] {
            break;
        }
    }

    score.push(0);
    for x in (tree_x + 1)..trees[0].len() {
        *score.last_mut().unwrap() += 1;
        if trees[tree_y][x] >= trees[tree_y][tree_x] {
            break;
        }
    }

    score.into_iter().fold(1, |acc, v| acc * v)
}

pub fn day8_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );

    let trees = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|n| n as i32))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut biggest_score = 0;

    for y in 0..trees.len() {
        for x in 0..trees[y].len() {
            let score = calc_scenic_score(&trees, x, y);
            if score > biggest_score {
                biggest_score = score;
            }
        }
    }

    biggest_score
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!((), day8_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day8_2("test"));
    }
}
