use std::{fs, io::stdin};

#[derive(Debug)]
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
        // println!(
        //     "Player {} rolls {}+{}+{} and moves to {} for a total of {}",
        //     turn + 1,
        //     a,
        //     b,
        //     c,
        //     data[turn].pos,
        //     data[turn].score
        // );
        turn = 1 - turn;
        if data[0].score >= 1000 || data[1].score >= 1000 {
            break;
        }
        // stdin().read_line(&mut String::new());
    }
    data[0].score.min(data[1].score) as u64 * die.times_rolled
}

pub fn day21_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let _data = fs::read_to_string(input_file);
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    fn t1() {
        assert_eq_ansval!(739785, day21_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!(444356092776315, day21_2("test"));
    }
}
