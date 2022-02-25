#![allow(dead_code, unused)]
use std::{
    fs,
    ops::{Add, AddAssign},
};

use regex::Regex;

use crate::AnsType;

// enum SnailfishVal {
//     Val(u32),
//     SNum(Box<SnailfishNum>),
// }

// struct SnailfishNum{
//     x: SnailfishVal,
//     y: SnailfishVal
// }

#[derive(Debug, Clone, Copy)]
enum SnailfishSymbol {
    OpenBraket,
    ClosedBraket,
    Val(i32),
}

impl From<&str> for SnailfishSymbol {
    fn from(from_str: &str) -> Self {
        match from_str {
            "[" => SnailfishSymbol::OpenBraket,
            "]" => SnailfishSymbol::ClosedBraket,
            s => SnailfishSymbol::Val(
                s.parse()
                    .expect("if not '[' or ']' symbol should parse to `i32`"),
            ),
        }
    }
}

impl AddAssign<i32> for SnailfishSymbol {
    fn add_assign(&mut self, rhs: i32) {
        if let SnailfishSymbol::Val(v) = self {
            *v += rhs;
        } else {
            panic!()
        }
    }
}
impl AddAssign for SnailfishSymbol {
    fn add_assign(&mut self, rhs: SnailfishSymbol) {
        if let (SnailfishSymbol::Val(s_val), SnailfishSymbol::Val(other)) = (self, rhs) {
            *s_val += other;
        } else {
            panic!()
        }
    }
}
impl Add for SnailfishSymbol {
    type Output = SnailfishSymbol;

    fn add(self, rhs: Self) -> Self::Output {
        if let (SnailfishSymbol::Val(s_val), SnailfishSymbol::Val(other)) = (self, rhs) {
            SnailfishSymbol::Val(s_val + other)
        } else {
            panic!()
        }
    }
}
impl Add<i32> for SnailfishSymbol {
    type Output = SnailfishSymbol;

    fn add(self, rhs: i32) -> Self::Output {
        match self {
            SnailfishSymbol::Val(v) => SnailfishSymbol::Val(v + rhs),
            _ => panic!(),
        }
    }
}

#[derive(Clone)]
struct SnailfishNum {
    data: Vec<SnailfishSymbol>,
}

impl std::fmt::Debug for SnailfishNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SnailfishNum: {}", self)
    }
}
impl std::fmt::Display for SnailfishSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SnailfishSymbol::OpenBraket => "[".to_owned(),
                SnailfishSymbol::ClosedBraket => "]".to_owned(),
                SnailfishSymbol::Val(v) => v.to_string(),
            }
        )
    }
}

impl Add for SnailfishNum {
    type Output = SnailfishNum;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        let mut ret_val = SnailfishNum::new(vec![SnailfishSymbol::OpenBraket]);
        ret_val.data.append(&mut self.data);
        ret_val.data.append(&mut rhs.data);
        ret_val.data.push(SnailfishSymbol::ClosedBraket);

        ret_val.reduce();

        ret_val
    }
}

impl std::fmt::Display for SnailfishNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut itr = self.data.iter().peekable();
        while let Some(sym) = itr.next() {
            write!(f, "{}", sym)?;
            match (sym, itr.peek()) {
                (SnailfishSymbol::ClosedBraket, Some(SnailfishSymbol::ClosedBraket)) => {}
                (
                    SnailfishSymbol::Val(_) | SnailfishSymbol::ClosedBraket,
                    Some(SnailfishSymbol::Val(_) | SnailfishSymbol::OpenBraket),
                ) => write!(f, ",")?,
                _ => (),
            }
        }
        Ok(())
    }
}

impl SnailfishNum {
    fn calc_magnitude(&self) -> i32 {
        let mut data = self.data.clone();
        'mag: loop {
            for i in 0..(data.len() - 1) {
                if let (SnailfishSymbol::Val(left), SnailfishSymbol::Val(right)) =
                    (data[i], data[i + 1])
                {
                    for _ in 0..4 {
                        data.remove(i - 1);
                    }
                    data.insert(i - 1, SnailfishSymbol::Val(3 * left + 2 * right));
                    continue 'mag;
                }
            }
            break;
        }
        match data[0] {
            SnailfishSymbol::Val(v) => v,
            _ => unimplemented!(),
        }
    }
    fn new(data: Vec<SnailfishSymbol>) -> Self {
        let mut val = Self { data };
        val.reduce();
        val
    }

    pub fn get_next_lit(&self, index: usize) -> Option<usize> {
        for i in index..self.data.len() {
            if let SnailfishSymbol::Val(_) = self.data[i] {
                return Some(i);
            }
        }
        None
    }

    pub fn reduce(&mut self) {
        'reduce: loop {
            let mut depth = 0;
            let mut last_num = None;

            for i in 0..self.data.len() {
                let curent_value = match self.data[i] {
                    SnailfishSymbol::OpenBraket => {
                        depth += 1;
                        continue;
                    }
                    SnailfishSymbol::ClosedBraket => {
                        depth -= 1;
                        continue;
                    }
                    SnailfishSymbol::Val(v) => v,
                };
                if let (Some(SnailfishSymbol::OpenBraket), Some(SnailfishSymbol::ClosedBraket)) =
                    (self.data.get(i), self.data.get(i + 1))
                {
                    self.data.remove(i);
                    self.data.remove(i);
                    continue 'reduce;
                } else if depth > 4 {
                    if let Some(SnailfishSymbol::Val(_)) = self.data.get(i + 1) {
                        if let Some(last) = last_num {
                            self.data[last] += curent_value
                        }
                        if let Some(index) = self.get_next_lit(i + 2) {
                            let num = self.data.remove(i + 1);
                            self.data[index - 1] += num;
                        } else {
                            self.data.pop();
                        }
                        for _ in 0..3 {
                            self.data.remove(i - 1);
                        }
                        self.data.insert(i - 1, SnailfishSymbol::Val(0));
                        // dbg!("continue");
                        continue 'reduce;
                    }
                    panic!();
                    // dbg!("continue");
                    continue 'reduce;
                } else {
                    last_num = Some(i)
                }
            }
            for i in 0..self.data.len() {
                let curent_value = match self.data[i] {
                    SnailfishSymbol::OpenBraket => {
                        depth += 1;
                        continue;
                    }
                    SnailfishSymbol::ClosedBraket => {
                        depth -= 1;
                        continue;
                    }
                    SnailfishSymbol::Val(v) => v,
                };
                if curent_value >= 10 {
                    self.data.remove(i);
                    self.data.insert(i, SnailfishSymbol::ClosedBraket);
                    self.data.insert(
                        i,
                        SnailfishSymbol::Val(if curent_value % 2 == 0 {
                            curent_value / 2
                        } else {
                            curent_value / 2 + 1
                        }),
                    );
                    self.data.insert(i, SnailfishSymbol::Val(curent_value / 2));
                    self.data.insert(i, SnailfishSymbol::OpenBraket);
                    // dbg!("continue");
                    continue 'reduce;
                }
            }
            break;
        }
    }

    pub fn reduce_nested_pair(&mut self, depth: usize) {}
}

pub fn day18_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let reg_expr = Regex::new(r"((?:\d+)|(?:\[)|(?:\]))").unwrap();
    let file_data = fs::read_to_string(input_file).unwrap();
    let mut data = file_data
        .split('\n')
        .map(|line| {
            SnailfishNum::new(
                reg_expr
                    .captures_iter(line)
                    .map(|cap| SnailfishSymbol::from(cap.get(1).unwrap().as_str()))
                    .collect(),
            )
        })
        .reduce(|f_val, new_val| (f_val + new_val));

    data.unwrap().calc_magnitude()
}

pub fn day18_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let reg_expr = Regex::new(r"((?:\d+)|(?:\[)|(?:\]))").unwrap();
    let file_data = fs::read_to_string(input_file).unwrap();
    let mut data = file_data
        .split('\n')
        .map(|line| {
            SnailfishNum::new(
                reg_expr
                    .captures_iter(line)
                    .map(|cap| SnailfishSymbol::from(cap.get(1).unwrap().as_str()))
                    .collect(),
            )
        })
        .collect::<Vec<_>>();

    let mut max_mag = 0;
    for a in 0..data.len() {
        for b in 0..data.len() {
            let sum = data[a].clone() + data[b].clone();
            let mag = sum.calc_magnitude();
            if mag > max_mag {
                max_mag = mag;
            }
        }
    }
    max_mag
}
