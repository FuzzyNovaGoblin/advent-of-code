#![allow(dead_code)]

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    default::default,
    fmt::Display,
    fs,
    io::stdin,
};

type PointPair = (u8, u8);

const POINTS: [(u8, u8); 19] = [
    (0, 0),
    (1, 0),
    (2, 0),
    (2, 1),
    (2, 2),
    (3, 0),
    (4, 0),
    (4, 1),
    (4, 2),
    (5, 0),
    (6, 0),
    (6, 1),
    (6, 2),
    (7, 0),
    (8, 0),
    (8, 1),
    (8, 2),
    (9, 0),
    (10, 0),
];

fn distance(a: PointPair, b: PointPair) -> i32 {
    let val = 10_f64.powi((a.0 as i32 - 2) / 2);
    if a == b {
        return 0;
    }

    // if a.0 == b.0 {
    //     a.1.abs_diff(b.1) as i32
    // } else {
    //     (a.1 + b.0.abs_diff(a.0)) as i32
    // }

    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as i32
    // +
    // ((a.0.abs_diff(b.0)*a.0.abs_diff(b.0) +a.1.abs_diff(b.1)*a.1.abs_diff(b.1)) as f32).sqrt() as i32

    // let x_dist = a.0.abs_diff(b.0);
    // let y_dist = if b.1 == 0 {
    //     a.1
    // } else if x_dist != 0 {
    //     a.1 + b.1
    // } else {
    //     a.1.abs_diff(b.1)
    // };
    // let add_v = if b.1 != 0 && b.0 != a.0 { 0 } else { 0 };
    // (x_dist + y_dist) as i32 + add_v
}

struct BoardStatePointsIter<'a> {
    letter: char,
    index: usize,
    state: &'a BoardState,
}

impl<'a> BoardStatePointsIter<'a> {
    fn new(state: &'a BoardState) -> Self {
        Self {
            letter: 'A',
            index: 0,
            state,
        }
    }
}

impl<'a> Iterator for BoardStatePointsIter<'a> {
    type Item = PointPair;

    fn next(&mut self) -> Option<Self::Item> {
        if self.letter == 'E' {}
        let ret_val = match self.letter {
            'A' => self.state.apos[self.index],
            'B' => self.state.bpos[self.index],
            'C' => self.state.cpos[self.index],
            'D' => self.state.dpos[self.index],
            _ => return None,
        };
        self.index += 1;
        if self.index > 1 {
            self.index = 0;
            self.letter = (self.letter as u8 + 1) as char;
        }
        Some(ret_val)
    }
}

#[derive(Eq, Default, Debug, Clone)]
struct BoardState {
    cost: u32,
    last: Option<Box<BoardState>>,
    steps: u32,
    apos: Vec<(u8, u8)>,
    bpos: Vec<(u8, u8)>,
    cpos: Vec<(u8, u8)>,
    dpos: Vec<(u8, u8)>,
}
impl std::hash::Hash for BoardState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.apos.hash(state);
        self.bpos.hash(state);
        self.cpos.hash(state);
        self.dpos.hash(state);
    }
}

impl Display for BoardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let parts = self.iter().collect::<HashSet<_>>();
        write!(f, "#############\n#")?;
        for x in 0..=10 {
            self.print_at_point(f, (x, 0))?;
        }
        write!(f, "#\n###")?;
        for x in (2..=8).step_by(2) {
            self.print_at_point(f, (x, 1))?;
            write!(f, "#")?;
        }
        write!(f, "##\n  #")?;
        for x in (2..=8).step_by(2) {
            self.print_at_point(f, (x, 2))?;
            write!(f, "#")?;
        }
        write!(f, "\n  #########")
    }
}

impl BoardState {
    fn print_at_point(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        point: PointPair,
    ) -> std::fmt::Result {
        // dbg!(point);
        match self.get_point_loc(point) {
            Some((c, _)) => write!(f, "{}", c),
            None => write!(f, "."),
        }
    }

    pub fn iter<'a>(&'a self) -> BoardStatePointsIter<'a> {
        BoardStatePointsIter::new(self)
    }

    fn get_point_loc(&self, point: PointPair) -> Option<(char, usize)> {
        let mut count = 0;
        for p in self.iter() {
            if p == point {
                return Some(((65 + (count / 2_u8)) as char, (count % 2) as usize));
            }
            count += 1;
        }
        None
    }

    fn remove_point(&mut self, point: PointPair) -> Option<char> {
        match self.get_point_loc(point) {
            Some(('A', index)) => {
                self.apos.remove(index);
                Some('A')
            }
            Some(('B', index)) => {
                self.bpos.remove(index);
                Some('B')
            }
            Some(('C', index)) => {
                self.cpos.remove(index);
                Some('C')
            }
            Some(('D', index)) => {
                self.dpos.remove(index);
                Some('D')
            }
            None => None,
            _ => unreachable!(),
        }
    }

    pub fn initialize(init_data: Vec<char>) -> Self {
        let mut count = 0;

        let mut ret_val = BoardState::default();

        for y in 1..=2 {
            for x in (2..=8).step_by(2) {
                let c = init_data[count];
                match c {
                    'A' => ret_val.apos.push((x, y)),
                    'B' => ret_val.bpos.push((x, y)),
                    'C' => ret_val.cpos.push((x, y)),
                    'D' => ret_val.dpos.push((x, y)),
                    _ => unreachable!(),
                }
                count += 1;
            }
        }
        ret_val
    }

    fn get_neighbors((start_x, start_y): PointPair) -> Vec<PointPair> {
        let mut ret_val = Vec::new();
        for x in sub_one_or_zero(start_x)..=start_x + 1 {
            for y in sub_one_or_zero(start_y)..=start_y + 1 {
                let tmp_point = (x, y);
                if (x != start_x && y != start_y) || (x, y) == (start_x, start_y) {
                    continue;
                }
                if POINTS.contains(&tmp_point) {
                    ret_val.push(tmp_point)
                }
            }
        }
        ret_val
    }

    // fn get_points(&self) -> Vec<PointPair> {
    //     let mut ret_val = Vec::new();
    //     ret_val.append(&mut self.apos.clone());
    //     ret_val.append(&mut self.bpos.clone());
    //     ret_val.append(&mut self.cpos.clone());
    //     ret_val.append(&mut self.dpos.clone());
    //     ret_val
    // }

    fn get_point_moves(&self, point: PointPair) -> Vec<PointPair> {
        let mut ret_val = BoardState::get_neighbors(point);
        'checked_points: for i in (0..ret_val.len()).rev() {
            for p in self.iter() {
                if p == ret_val[i] {
                    ret_val.remove(i);
                    continue 'checked_points;
                }
            }
        }
        ret_val
    }

    fn calc_distances(&self) -> i32 {
        let a = (distance((2, 1), self.apos[0]) + distance((2, 2), self.apos[1]))
            .min(distance((2, 1), self.apos[1]) + distance((2, 2), self.apos[0]));

        let b = (distance((4, 1), self.bpos[0]) + distance((4, 2), self.bpos[1]))
            .min(distance((4, 1), self.bpos[1]) + distance((4, 2), self.bpos[0]));

        let c = (distance((6, 1), self.cpos[0]) + distance((6, 2), self.cpos[1]))
            .min(distance((6, 1), self.cpos[1]) + distance((6, 2), self.cpos[0]));
        let d = (distance((8, 1), self.dpos[0]) + distance((8, 2), self.dpos[1]))
            .min(distance((8, 1), self.dpos[1]) + distance((8, 2), self.dpos[0]));

        // dbg!(a, b, c, d);

        a + b + c + d
        // a + b * 10 + c * 100 + d * 1000
    }

    fn print_last_moves(&self) {
        if let Some(pat) = &self.last {
            pat.print_last_moves();
        }
        println!("\n{}\n", self);
    }

    fn print_last_moves_reversed(&self) {
        println!("\n{}\n", self);
        if let Some(pat) = &self.last {
            pat.print_last_moves_reversed();
        }
    }

    fn into_moves(self) -> Vec<Self> {
        let mut ret_val = Vec::new();

        for p in self.iter() {
            let mut new_state = self.clone();
            if let Some(letter) = new_state.remove_point(p) {
                for moves in self.get_point_moves(p) {
                    let mut new_state = new_state.clone();
                    new_state.last = Some(Box::new(self.clone()));
                    match letter {
                        'A' => {
                            new_state.apos.push(moves);
                            new_state.steps += 1;
                            new_state.cost += 1;
                        }
                        'B' => {
                            new_state.bpos.push(moves);
                            new_state.steps += 1;
                            new_state.cost += 10;
                        }
                        'C' => {
                            new_state.cpos.push(moves);
                            new_state.steps += 1;
                            new_state.cost += 100;
                        }
                        'D' => {
                            new_state.dpos.push(moves);
                            new_state.steps += 1;
                            new_state.cost += 1000;
                        }
                        _ => unreachable!(),
                    }
                    ret_val.push(new_state);
                }
            }
        }

        ret_val
    }
}

fn sub_one_or_zero(v: u8) -> u8 {
    match v {
        0 => 0,
        v => v - 1,
    }
}

impl PartialEq for BoardState {
    fn eq(&self, other: &Self) -> bool {
        ((self.apos[0] == other.apos[0] && self.apos[1] == other.apos[1])
            || (self.apos[0] == other.apos[1] && self.apos[1] == other.apos[0]))
            && ((self.bpos[0] == other.bpos[0] && self.bpos[1] == other.bpos[1])
                || (self.bpos[0] == other.bpos[1] && self.bpos[1] == other.bpos[0]))
            && ((self.cpos[0] == other.cpos[0] && self.cpos[1] == other.cpos[1])
                || (self.cpos[0] == other.cpos[1] && self.cpos[1] == other.cpos[0]))
            && ((self.dpos[0] == other.dpos[0] && self.dpos[1] == other.dpos[1])
                || (self.dpos[0] == other.dpos[1] && self.dpos[1] == other.dpos[0]))
    }
}

impl Ord for BoardState {
    fn cmp(&self, other: &Self) -> Ordering {
        // match self.steps.cmp(&other.steps) {
        //     Ordering::Equal => match self.calc_distances().cmp(&other.calc_distances()) {
        //         Ordering::Equal => self.cost.cmp(&other.cost),
        //         o => return o,
        //     },
        //     o => o,
        // }
        match self.cost.cmp(&other.cost) {
            Ordering::Equal => match self.steps.cmp(&other.steps) {
                // Ordering::Equal => self.calc_distances().cmp(&other.calc_distances()),
                o => return o,
            },
            o => o,
        }

        // match self.calc_distances().cmp(&other.calc_distances()) {
        //     Ordering::Equal => self.cost.cmp(&other.cost),

        //     o => o,
        // }
    }
}

impl PartialOrd for BoardState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // match self.steps.partial_cmp(&other.steps) {
        //     Some(Ordering::Equal) | None => {
        //         match self.calc_distances().partial_cmp(&other.calc_distances()) {
        //             Some(Ordering::Equal) => self.cost.partial_cmp(&other.cost),
        //             Some(o) => Some(o),
        //             None => None,
        //         }
        //     }
        //     o => o,
        // }

        // match self.calc_distances().partial_cmp(&other.calc_distances()) {
        //     Some(Ordering::Equal) => self.cost.partial_cmp(&other.cost),

        //     o => o,
        // }

        match self.calc_distances().partial_cmp(&other.calc_distances()) {
            Some(Ordering::Equal) | None => match self.cost.partial_cmp(&other.cost) {
                Some(Ordering::Equal) => self.steps.partial_cmp(&other.steps),
                Some(o) => Some(o),
                None => None,
            },
            o => o,
        }
    }
}

pub fn day23_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let data = fs::read_to_string(input_file)
        .unwrap()
        .chars()
        .filter(|c| matches!(c, 'A'..='D'))
        .collect::<Vec<_>>();

    let mut states: BinaryHeap<Reverse<BoardState>> = default();
    states.push(Reverse(BoardState::initialize(data)));

    // let mut been_to = HashSet::<BoardState>::new();
    // been_to.insert(states.peek().unwrap().0.clone());

    let mut found_one = false;
    let mut min_found = 0;
    let mut min_found_state = None;

    while let Some(Reverse(state)) = states.pop() {
        // println!("{}", state.calc_distances());
        // todo!();
        if state.calc_distances() == 0 {
            println!("found {}", state.cost);
            if found_one {
                if min_found > state.cost {
                    min_found = state.cost;
                    min_found_state = Some(state);
                }
            } else {
                min_found = state.cost;
                min_found_state = Some(state);
                found_one = true;
            }
            continue;
        }

        // todo!();
        // dbg!(&been_to.len());
        // count += 1;
        // if count > 100 {
        //     count = 0;
        //     println!("{}\n\n", state);

        //     // for b in been_to {
        //     //     println!("{}\n\n", b);
        //     // }
        //     // todo!();
        // }
        // if !found_one {
        for m in state.into_moves() {
            // if !been_to.contains(&m) {
            //     been_to.insert(m.clone());
            states.push(Reverse(m))
            // }
        }
        // }
    }

    // min_found_state.unwrap().print_last_moves_reversed();
    // loop{
    //     if let Some(pat) = min_found_state {
    //         println!("{}", pat);
    //         min_found_state = Some(pat);
    //     }
    //     else{
    //         break;
    //     }
    // }
    min_found
}

pub fn day23_2(file_name: &str) -> impl crate::AnsType {
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
    #[ignore]
    fn t1() {
        assert_eq_ansval!((12521), day23_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day23_2("test"));
    }
}
