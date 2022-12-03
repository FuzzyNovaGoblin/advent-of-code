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
enum RPS {
    Rock,
    Paper,
    Scissors,
}
impl RPS {
    fn from_letter(letter: char) -> RPS {
        match letter {
            'A' => RPS::Rock,
            'B' => RPS::Paper,
            'C' => RPS::Scissors,
            'X' => RPS::Rock,
            'Y' => RPS::Paper,
            'Z' => RPS::Scissors,
            c => unreachable!("{}", c),
        }
    }

    fn corisponding_outcome(&self, desired: Outcome) -> RPS {
        match (self, desired) {
            (RPS::Rock, Outcome::Lose) => RPS::Scissors,
            (RPS::Rock, Outcome::Draw) => RPS::Rock,
            (RPS::Rock, Outcome::Win) => RPS::Paper,
            (RPS::Paper, Outcome::Lose) => RPS::Rock,
            (RPS::Paper, Outcome::Draw) => RPS::Paper,
            (RPS::Paper, Outcome::Win) => RPS::Scissors,
            (RPS::Scissors, Outcome::Lose) => RPS::Paper,
            (RPS::Scissors, Outcome::Draw) => RPS::Scissors,
            (RPS::Scissors, Outcome::Win) => RPS::Rock,
        }
    }

    fn fight(&self, other: &RPS) -> u32 {
        match (self, other) {
            (RPS::Rock, RPS::Rock) => 1 + 3,
            (RPS::Rock, RPS::Paper) => 1,
            (RPS::Rock, RPS::Scissors) => 1 + 6,
            (RPS::Paper, RPS::Rock) => 2 + 6,
            (RPS::Paper, RPS::Paper) => 2 + 3,
            (RPS::Paper, RPS::Scissors) => 2,
            (RPS::Scissors, RPS::Rock) => 3,
            (RPS::Scissors, RPS::Paper) => 3 + 6,
            (RPS::Scissors, RPS::Scissors) => 3 + 3,
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
                RPS::from_letter(chars.next().unwrap()),
                RPS::from_letter(chars.nth(1).unwrap()),
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
                RPS::from_letter(chars.next().unwrap()),
                Outcome::from_letter(chars.nth(1).unwrap()),
            )
        })
        .map(|(other, outcome)| (other, other.corisponding_outcome(outcome)))
        .fold(0, |sum, (other, you)| you.fight(&other) + sum)
}
