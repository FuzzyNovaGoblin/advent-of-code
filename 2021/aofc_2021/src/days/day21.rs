use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct GameState {
    player1: Player,
    player2: Player,
}

impl GameState {
    fn get_turn_player(&mut self, turn: u8) -> &mut Player {
        match turn {
            0 => &mut self.player1,
            1 => &mut self.player2,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Player {
    pos: u32,
    pub score: u32,
}

impl Player {
    fn new(pos: u32) -> Self {
        Self { pos, score: 0 }
    }
    fn add_roll(&mut self, roll: u32) {
        self.pos += roll;
        self.pos %= 10;
        if self.pos == 0 {
            self.pos = 10;
        }
        self.score += self.pos as u32;
    }
}

#[derive(Debug)]
struct DeterministicDice {
    next_roll: u32,
    times_rolled: u64,
}
impl DeterministicDice {
    pub(crate) fn new(start: u32) -> DeterministicDice {
        DeterministicDice {
            next_roll: start,
            times_rolled: 0,
        }
    }
}

impl Iterator for DeterministicDice {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.times_rolled += 1;
        let ret = self.next_roll;
        self.next_roll += 1;
        if self.next_roll > 100 {
            self.next_roll = 1;
        }
        Some(ret)
    }
}

pub fn day21_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let mut data = fs::read_to_string(input_file)
        .unwrap()
        .split('\n')
        .map(|line| Player::new(line.chars().last().unwrap() as u32 - 48))
        .collect::<Vec<_>>();
    let mut turn = 0;
    let mut die = DeterministicDice::new(1);

    while let (Some(a), Some(b), Some(c)) = (die.next(), die.next(), die.next()) {
        data[turn].add_roll(a + b + c);
        turn = 1 - turn;
        if data[0].score >= 1000 || data[1].score >= 1000 {
            break;
        }
    }
    data[0].score.min(data[1].score) as u64 * die.times_rolled
}

const OUTCOMES: [(u8, u8); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

pub fn day21_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let mut data = fs::read_to_string(input_file)
        .unwrap()
        .split('\n')
        .map(|line| Player::new(line.chars().last().unwrap() as u32 - 48))
        .collect::<Vec<_>>();
    let mut wins = (0_u64, 0_u64);
    let player1 = data.remove(0);
    let player2 = data.remove(0);
    let mut game_states = HashMap::<GameState, u64>::new();
    game_states.insert(GameState { player1, player2 }, 1);
    let mut turn = 0;

    while !game_states.is_empty() {
        let mut new_game_states = HashMap::<GameState, u64>::new();

        for (state, count) in game_states {
            if state.player1.score >= 21 {
                wins.0 += count;
                continue;
            }
            if state.player2.score >= 21 {
                wins.1 += count;
                continue;
            }
            for (roll, roll_count) in OUTCOMES {
                let mut state = state.clone();
                state.get_turn_player(turn).add_roll(roll as u32);
                let e = new_game_states.entry(state).or_insert(0);
                *e += count * roll_count as u64;
            }
        }

        game_states = new_game_states;
        turn = 1 - turn;
    }
    wins.0.max(wins.1)
}
