use crate::point_map::CordPoint;
use std::{default::default, fs};

#[derive(Debug, Clone)]
struct TargetArea {
    x_min: i128,
    x_max: i128,
    y_min: i128,
    y_max: i128,
}

impl TargetArea {
    fn is_within(&self, point: &CordPoint) -> bool {
        point.x >= self.x_min
            && point.x <= self.x_max
            && point.y >= self.y_min
            && point.y <= self.y_max
    }
}

impl From<Vec<Vec<i128>>> for TargetArea {
    fn from(other: Vec<Vec<i128>>) -> Self {
        Self {
            x_min: other[0][0],
            x_max: other[0][1],
            y_min: other[1][0],
            y_max: other[1][1],
        }
    }
}

#[derive(Debug)]
struct Probe {
    x_vel: i128,
    y_vel: i128,
    pos: CordPoint,
}

impl Probe {
    fn new(x_vel: i128, y_vel: i128) -> Self {
        Self {
            x_vel,
            y_vel,
            pos: default(),
        }
    }

    fn step(&mut self) {
        // step 1
        self.pos.x += self.x_vel;
        // step 2
        self.pos.y += self.y_vel;
        // step 3
        match self.x_vel.cmp(&0) {
            std::cmp::Ordering::Less => self.x_vel -= 1,
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => self.x_vel += 1,
        }
        // step 4
        self.y_vel -= 1;
    }
}

pub fn day17_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let target_area: TargetArea = fs::read_to_string(input_file)
        .unwrap()
        .chars()
        .skip(12)
        .collect::<String>()
        .split(',')
        .map(|s| {
            s.chars()
                .skip(3)
                .to_owned()
                .collect::<String>()
                .split("..")
                .map(|num_str| num_str.parse().expect("num from string"))
                .collect()
        })
        .collect::<Vec<Vec<_>>>()
        .into();

    let mut max_yval = 0;

    for x in 0..target_area.x_max {
        for y in target_area.y_min..(target_area.y_max.abs() * 3) {
            let mut current_max_yval = 0;
            let mut valid: bool = false;
            let mut probe = Probe::new(x as i128, y as i128);
            while (probe.x_vel != 0
                && ((probe.pos.x <= target_area.x_max && probe.x_vel >= 1)
                    || (probe.pos.x >= target_area.x_min && probe.x_vel <= 1)))
                || (probe.pos.y >= target_area.y_min)
            {
                if probe.pos.y > current_max_yval {
                    current_max_yval = probe.pos.y;
                }
                probe.step();
                if target_area.is_within(&probe.pos) {
                    valid = true;
                    break;
                }
            }
            if valid && max_yval < current_max_yval {
                max_yval = current_max_yval;
            }
        }
    }
    max_yval
}

pub fn day17_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let target_area: TargetArea = fs::read_to_string(input_file)
        .unwrap()
        .chars()
        .skip(12)
        .collect::<String>()
        .split(',')
        .map(|s| {
            s.chars()
                .skip(3)
                .to_owned()
                .collect::<String>()
                .split("..")
                .map(|num_str| num_str.parse().expect("num from string"))
                .collect()
        })
        .collect::<Vec<Vec<_>>>()
        .into();

    let mut valid_velocities = 0;

    for x in 0..=target_area.x_max {
        for y in (target_area.y_max.abs() * -2)..=(target_area.y_max.abs() * 2) {
            let mut probe = Probe::new(x as i128, y as i128);
            while (probe.x_vel != 0
                && ((probe.pos.x <= target_area.x_max && probe.x_vel >= 1)
                    || (probe.pos.x >= target_area.x_min && probe.x_vel <= 1)))
                || (probe.pos.y >= target_area.y_min)
            {
                probe.step();

                if target_area.is_within(&probe.pos) {
                    valid_velocities += 1;
                    break;
                }
            }
        }
    }
    valid_velocities
}
