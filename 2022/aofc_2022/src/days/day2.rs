use std::fs;

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn from_letter(letter: char) -> Outcome {
        match letter {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            c => unreachable!("{}", c),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}
impl Rps {
    fn from_letter(letter: char) -> Rps {
        match letter {
            'A' => Rps::Rock,
            'B' => Rps::Paper,
            'C' => Rps::Scissors,
            'X' => Rps::Rock,
            'Y' => Rps::Paper,
            'Z' => Rps::Scissors,
            c => unreachable!("{}", c),
        }
    }

    fn corisponding_outcome(&self, desired: Outcome) -> Rps {
        match (self, desired) {
            (Rps::Rock, Outcome::Lose) => Rps::Scissors,
            (Rps::Rock, Outcome::Draw) => Rps::Rock,
            (Rps::Rock, Outcome::Win) => Rps::Paper,
            (Rps::Paper, Outcome::Lose) => Rps::Rock,
            (Rps::Paper, Outcome::Draw) => Rps::Paper,
            (Rps::Paper, Outcome::Win) => Rps::Scissors,
            (Rps::Scissors, Outcome::Lose) => Rps::Paper,
            (Rps::Scissors, Outcome::Draw) => Rps::Scissors,
            (Rps::Scissors, Outcome::Win) => Rps::Rock,
        }
    }

    fn fight(&self, other: &Rps) -> u32 {
        match (self, other) {
            (Rps::Rock, Rps::Rock) => 1 + 3,
            (Rps::Rock, Rps::Paper) => 1,
            (Rps::Rock, Rps::Scissors) => 1 + 6,
            (Rps::Paper, Rps::Rock) => 2 + 6,
            (Rps::Paper, Rps::Paper) => 2 + 3,
            (Rps::Paper, Rps::Scissors) => 2,
            (Rps::Scissors, Rps::Rock) => 3,
            (Rps::Scissors, Rps::Paper) => 3 + 6,
            (Rps::Scissors, Rps::Scissors) => 3 + 3,
        }
    }
}

pub fn day2_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file);
    data.unwrap()
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            (
                Rps::from_letter(chars.next().unwrap()),
                Rps::from_letter(chars.nth(1).unwrap()),
            )
        })
        .fold(0, |sum, (other, you)| you.fight(&other) + sum)
}

pub fn day2_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file);
    data.unwrap()
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            (
                Rps::from_letter(chars.next().unwrap()),
                Outcome::from_letter(chars.nth(1).unwrap()),
            )
        })
        .map(|(other, outcome)| (other, other.corisponding_outcome(outcome)))
        .fold(0, |sum, (other, you)| you.fight(&other) + sum)
}
