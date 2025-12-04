use std::{fmt::Display, fs, ops};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => f.write_fmt(format_args! {"L"}),
            Direction::Right => f.write_fmt(format_args! {"R"}),
        }
    }
}

#[derive(Debug)]
struct Turn {
    dir: Direction,
    mag: i32,
}

impl From<&str> for Turn {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();

        let dir = match chars.next().unwrap() {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => unreachable!(),
        };
        Turn {
            dir,
            mag: chars.collect::<String>().parse::<i32>().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Dial {
    pos: i64,
    past_0_count: u32,
}

impl Dial {
    fn new(pos: i64) -> Self {
        Self {
            pos,
            past_0_count: 0,
        }
    }
}

impl ops::AddAssign<Turn> for Dial {
    fn add_assign(&mut self, rhs: Turn) {
        let mut new_pos = match rhs.dir {
            Direction::Left => self.pos - rhs.mag as i64,
            Direction::Right => self.pos + rhs.mag as i64,
        };

        if self.pos == 0 && new_pos < 0 {
            self.past_0_count -= 1
        }
        if rhs.dir == Direction::Left && new_pos % 100 == 0 {
            self.past_0_count += 1
        }

        if new_pos == 100 {
            new_pos = 0;
            self.past_0_count += 1;
        }
        while new_pos < 0 {
            new_pos += 100;
            self.past_0_count += 1;
        }
        while new_pos > 99 {
            new_pos -= 100;
            self.past_0_count += 1;
        }
        self.pos = new_pos
    }
}

// impl ops::Add<Turn> for Dial {
//     type Output = Dial;
//     fn add(mut self, rhs: Turn) -> Self::Output {
//         let mut new_pos = match rhs.dir {
//             Direction::Left => self.pos - rhs.mag as i64,
//             Direction::Right => self.pos + rhs.mag as i64,
//         };
//         if new_pos % 100 == self.pos {
//             self.past_0_count += 1;
//         }
//         while new_pos < 0 {
//             new_pos += 100;
//             self.past_0_count += 1;
//         }
//         while new_pos > 99 {
//             new_pos -= 100;
//             self.past_0_count += 1;
//         }
//         Dial {
//             pos: new_pos,
//             ..self
//         }
//     }
// }

pub fn day1_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2025/input/{}",
        env!("ADVENT_OF_CODE_2025"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();
    let turns = data.lines().into_iter().map(Turn::from).collect::<Vec<_>>();

    let mut dial = Dial::new(50);
    let mut count = 0;
    for t in turns {
        dial += t;
        if dial.pos == 0 {
            count += 1;
        }
        // dbg!(&dial);
    }

    count
}

pub fn day1_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2025/input/{}",
        env!("ADVENT_OF_CODE_2025"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();
    let turns = data.lines().into_iter().map(Turn::from).collect::<Vec<_>>();

    let mut dial = Dial::new(50);

    for t in turns {
        dial += t;
    }

    dial.past_0_count
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!((), day1_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day1_2("test"));
    }
}
