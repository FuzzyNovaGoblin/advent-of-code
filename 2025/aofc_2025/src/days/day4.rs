use std::{fmt::Display, fs, usize};

#[derive(Debug, Clone, Copy)]
enum PointType {
    Roll,
    Empty,
    None,
}

impl Display for PointType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PointType::Roll => '@',
                PointType::Empty => '.',
                PointType::None => '_',
            }
        )
    }
}

impl From<char> for PointType {
    fn from(value: char) -> Self {
        match value {
            '@' => PointType::Roll,
            '.' => PointType::Empty,
            _ => PointType::None,
        }
    }
}

#[derive(Debug)]
struct PaperMap {
    points: Vec<Vec<PointType>>,
}

impl PaperMap {
    fn new<S: AsRef<str>>(points_str: S) -> Self {
        Self {
            points: points_str
                .as_ref()
                .lines()
                .map(|l| l.trim().chars().map(PointType::from).collect())
                .collect(),
        }
    }
    fn get_neibors(&self, coord: [usize; 2]) -> [PointType; 8] {
        let mut ret = [PointType::None; 8];
        let mut count = 0;
        for i in (-1..2).enumerate() {
            for j in (-1..2).enumerate() {
                if i.1 == 0 && j.1 == 0 {
                    continue;
                }
                let (y, x) = (coord[0] as i32 + i.1, coord[1] as i32 + j.1);
                if y < 0
                    || x < 0
                    || y >= self.points.len() as i32
                    || x >= self.points[y as usize].len() as i32
                {
                    ret[count] = PointType::None
                } else {
                    ret[count] = self.points[y as usize][x as usize];
                }
                count += 1;
            }
        }

        ret
    }

    fn can_remove(&self, coord: [usize; 2]) -> bool {
        matches!(self.points[coord[0]][coord[1]], PointType::Roll)
            && self
                .get_neibors([coord[0], coord[1]])
                .iter()
                .filter(|p| matches!(p, PointType::Roll))
                .count()
                < 4
    }

    fn remove_valid(&mut self) -> u32 {
        let mut removed_count = 0;

        for y in 0..self.points.len() {
            for x in 0..self.points[y].len() {
                if self.can_remove([y, x]) {
                    removed_count += 1;
                    self.points[y][x] = PointType::Empty
                }
            }
        }

        removed_count
    }
}

pub fn day4_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2025/input/{}",
        env!("ADVENT_OF_CODE_2025"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();
    // let data = if run_test{
    //     include_str!("../../input/day4")
    // }else{
    //     include_str!("../../input/day4")
    // };

    let map = PaperMap::new(data);

    let mut valid = 0;

    for y in 0..map.points.len() {
        for x in 0..map.points[y].len() {
            if map.can_remove([y, x]) {
                valid += 1;
            }
        }
    }

    valid
}

pub fn day4_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2025/input/{}",
        env!("ADVENT_OF_CODE_2025"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();
    let mut map = PaperMap::new(data);

    let mut removed_count = 0;
    loop {
        let r = map.remove_valid();
        if r == 0 {
            break;
        }
        removed_count += r;
    }

    removed_count
}

fn _print_neighbors(points: [PointType; 8]) {
    let mut count = 0;
    for i in 0..3 {
        print!("———————\n|");
        for j in 0..3 {
            if i == 1 && j == 1 {
                print!("*|");
            } else {
                print!("{}|", points[count]);
                count += 1
            }
        }
        println!();
    }
    println!("———————");
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    fn t1() {
        assert_eq_ansval!(1351, day4_1("day4"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!(43, day4_2("test"));
    }
}
