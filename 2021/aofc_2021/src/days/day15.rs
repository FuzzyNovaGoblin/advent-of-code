use crate::{
    AnsType,
    dijkstra::DijkstraDistance::{self, *},
};
use std::{
    collections::{HashMap, HashSet, BinaryHeap},
    default::default,
    fmt::Debug,
    fs, cmp::Ordering,
};

use crate::point_map::{CordPoint, PointMap};

#[derive(PartialEq, Default, Debug)]
struct TravelPoint {
    pub risk_level: DijkstraDistance,
    pub pos: CordPoint,
}

impl TravelPoint {
    fn new(risk_level: DijkstraDistance, pos: CordPoint) -> Self {
        Self {
            risk_level,
            pos,
        }
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

#[allow(dead_code)]
pub fn print_path(been_to: &HashSet<CordPoint>, point_map: &PointMap<u32>) {
    let mut point_map: PointMap<String> = point_map.clone().into();
    for point in been_to.iter() {
        point_map[*point] = "@".into();
    }
    eprintln!("{}", point_map);
}

pub fn day15_1(file_name: &str) -> impl AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let _data = fs::read_to_string(input_file);

    let mut point_map = PointMap::default();
    let mut distance_map: HashMap<CordPoint, DijkstraDistance> = default();
    let mut checked_pos: HashSet<CordPoint> = default();

    distance_map.insert((0, 0), Distance(0));

    for (y, line) in _data.unwrap().split('\n').enumerate() {
        for (x, char) in line.chars().enumerate() {
            point_map[(x, y)] = char.to_digit(10).unwrap();
        }
    }

    let dimentions = point_map.get_dimentions();
    let last_point: CordPoint = (dimentions.0 - 1, dimentions.1 - 1);

    let mut pos_queue = BinaryHeap::<TravelPoint>::new();
    pos_queue.push(TravelPoint::new(Distance(0), (0,0)));

    while checked_pos.len() < point_map.len() {
        let (current_point, current_distance):(CordPoint, DijkstraDistance) = {
                let pop_val = pos_queue.pop().unwrap();
                (pop_val.pos, pop_val.risk_level)
        };
        assert_ne!(((0, 0), Infinity), (current_point, current_distance));

        checked_pos.insert(current_point);
        if current_point == last_point {
            break;
        }

        for next_point in point_map.get_boardering_points(current_point, 0b0_1010_1010) {
            let distance = point_map[next_point];
            let new_distance = current_distance + distance;
            if &new_distance < distance_map.entry(next_point).or_default() {
                distance_map.insert(next_point, current_distance + distance);
                pos_queue.push(TravelPoint::new(current_distance + distance, next_point));
            }
        }
    }

    distance_map[&last_point]
}

pub fn day15_2(file_name: &str) -> impl AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let _data = fs::read_to_string(input_file);

    let mut point_map = PointMap::default();
    let mut distance_map: HashMap<CordPoint, DijkstraDistance> = default();
    let mut checked_pos: HashSet<CordPoint> = default();

    distance_map.insert((0, 0), Distance(0));

    for (y, line) in _data.unwrap().split('\n').enumerate() {
        for (x, char) in line.chars().enumerate() {
            point_map[(x, y)] = char.to_digit(10).unwrap();
        }
    }

    let dimentions = point_map.get_dimentions();

    for outer_x in 0..5 {
        for outer_y in 0..5 {
            if outer_x == 0 && outer_y == 0{
                continue;
            }
            for inner_x in 0..dimentions.0 {
                for inner_y in 0..dimentions.1 {
                    let current_pos = (inner_x+(dimentions.0*outer_x), inner_y+(dimentions.1*outer_y));
                    point_map[current_pos] = point_map[(inner_x,inner_y)] + (outer_x+outer_y) as u32;
                    if point_map[current_pos] >= 10{
                        point_map[current_pos] -= 9;
                    }
                }
            }
        }
    }

    let dimentions = point_map.get_dimentions();



    let last_point: CordPoint = (dimentions.0 - 1, dimentions.1 - 1);
    let mut pos_queue = BinaryHeap::<TravelPoint>::new();
    pos_queue.push(TravelPoint::new(Distance(0), (0,0)));

    while checked_pos.len() < point_map.len() {
        let (current_point, current_distance):(CordPoint, DijkstraDistance) = {
                let pop_val = pos_queue.pop().unwrap();
                (pop_val.pos, pop_val.risk_level)
        };

        assert_ne!(((0, 0), Infinity), (current_point, current_distance));

        checked_pos.insert(current_point);
        if current_point == last_point {
            break;
        }

        for next_point in point_map.get_boardering_points(current_point, 0b0_1010_1010) {
            let distance = point_map[next_point];
            let new_distance = current_distance + distance;
            if &new_distance < distance_map.entry(next_point).or_default() {
                distance_map.insert(next_point, current_distance + distance);
                pos_queue.push(TravelPoint::new(current_distance + distance, next_point));
            }
        }
    }

    distance_map[&last_point]
}
