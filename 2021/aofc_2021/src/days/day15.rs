use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    default::default,
    fs, fmt::Display,
};

use crate::point_map::{CordPoint, PointMap};

#[derive(PartialEq, Default, Debug)]
struct TravelPoint {
    pub risk_level: u32,
    pub pos: CordPoint,
    pub been_to: HashSet<CordPoint>,
}

impl TravelPoint {
    fn new(risk_level: u32, pos: CordPoint, been_to: HashSet<CordPoint>) -> Self {
        Self {
            risk_level,
            pos,
            been_to,
        }
    }

    pub fn print_path(&self, point_map: PointMap<u32>){

    }
}


impl PartialOrd for TravelPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.risk_level.partial_cmp(&self.risk_level)
    }
}

impl Eq for TravelPoint {}

impl Ord for TravelPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk_level.cmp(&self.risk_level)
    }
}

pub fn day15_1(file_name: &str) -> impl std::fmt::Debug {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let _data = fs::read_to_string(input_file);

    let mut point_map = PointMap::default();
    for (y, line) in _data.unwrap().split("\n").enumerate() {
        for (x, char) in line.chars().enumerate() {
            point_map[(x, y)] = char.to_digit(10).unwrap();
        }
    }
    let dimentions = point_map.get_dimentions();
    let mut pos_queue = BinaryHeap::<TravelPoint>::new();
    pos_queue.push(default());

    loop {
        let mut point = pos_queue.pop().expect("no points remain");
        if point.pos == (dimentions.0 - 1, dimentions.1 - 1) {
            return point.risk_level;
        }
        {
            // println!("{}", point);
            point.been_to.insert(point.pos);
            for p in point_map.get_cross_adjacent_points(point.pos) {
                if !point.been_to.contains(&p) {
                    pos_queue.push(TravelPoint::new(
                        point.risk_level + point_map[p],
                        p,
                        point.been_to.clone(),
                    ));
                }
            }
        }
    }
}

pub fn day15_2(file_name: &str) -> impl std::fmt::Debug {
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
    use crate::assert_eq_dbgfmt;

    #[test]
    fn t1() {
        assert_eq_dbgfmt!(40, day15_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_dbgfmt!((), day15_2("test"));
    }
}
