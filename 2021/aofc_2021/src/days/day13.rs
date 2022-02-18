use std::fs;

use crate::{
    days::day13::fold_map::{FoldMap, MapFold, SplitDirection},
    point_map::CordPoint,
};

mod fold_map {
    use std::collections::HashMap;
    use std::{fmt, ops};

    use crate::point_map::CordPoint;

    #[derive(Default)]
    pub struct FoldMap {
        points: HashMap<CordPoint, bool>,
        lowest_point: CordPoint,
        greatest_point: CordPoint,
    }

    impl IntoIterator for FoldMap {
        type Item = (CordPoint, bool);

        type IntoIter = std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            self.points.into_iter().collect::<Vec<_>>().into_iter()
        }
    }

    impl std::fmt::Debug for FoldMap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for y in 0..=self.greatest_point.1 {
                for x in 0..=self.greatest_point.0 {
                    write!(
                        f,
                        "{}",
                        match self.points.get(&(x, y)) {
                            Some(v) =>
                                if *v {
                                    '#'
                                } else {
                                    '.'
                                },
                            None => '.',
                        }
                    )?;
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }

    impl FoldMap {
        pub fn new(new_points: Vec<CordPoint>) -> Self {
            let mut tmp = Self::default();
            for p in new_points {
                tmp[p] = true;
            }
            tmp
        }
    }

    pub trait MapFold {
        fn fold_h(&mut self, x_val: usize);
        fn fold_v(&mut self, y_val: usize);
    }

    impl ops::Index<CordPoint> for FoldMap {
        type Output = bool;

        fn index(&self, index: CordPoint) -> &Self::Output {
            &self.points[&index]
        }
    }
    impl ops::IndexMut<CordPoint> for FoldMap {
        fn index_mut(&mut self, index: CordPoint) -> &mut Self::Output {
            if index.0 > self.greatest_point.0 {
                self.greatest_point.0 = index.0
            }
            if index.1 > self.greatest_point.1 {
                self.greatest_point.1 = index.1
            }
            if index.0 < self.lowest_point.0 {
                self.lowest_point.0 = index.0
            }
            if index.1 < self.lowest_point.1 {
                self.lowest_point.1 = index.1
            }
            self.points.entry(index).or_insert(false)
        }
    }

    impl MapFold for FoldMap {
        fn fold_h(&mut self, x_val: usize) {
            let new_points = self
                .points
                .drain_filter(|point, _v| point.0 > x_val)
                .collect::<Vec<_>>();

            self.greatest_point.0 = x_val - 1;
            for ((x, y), _) in new_points {
                self.points.insert((x_val + x_val - x, y), true);
            }
        }

        fn fold_v(&mut self, y_val: usize) {
            let new_points = self
                .points
                .drain_filter(|point, _val| point.1 > y_val)
                .collect::<Vec<_>>();
            self.greatest_point.1 = y_val - 1;
            for ((x, y), _) in new_points {
                self.points.insert((x, y_val + y_val - y), true);
            }
        }
    }
    pub enum SplitDirection {
        X(usize),
        Y(usize),
    }
}

#[allow(dead_code)]
fn default<T: Default>() -> T {
    T::default()
}

pub fn day13_1(file_name: &str) -> impl std::fmt::Debug {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let (point_data, split_data) = {
        let both = fs::read_to_string(input_file)
            .unwrap()
            .split("\n\n")
            .map(|v| v.to_owned())
            .collect::<Vec<_>>();

        (
            both[0]
                .split("\n")
                .map(|line| -> CordPoint {
                    let x_and_y = line
                        .split(",")
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();
                    (x_and_y[0], x_and_y[1])
                })
                .collect::<Vec<_>>(),
            both[1]
                .split("\n")
                .map(|line| {
                    let parts = line.split("=").collect::<Vec<_>>();
                    if parts[0].chars().last().unwrap() == 'x' {
                        SplitDirection::X(parts[1].parse().unwrap())
                    } else {
                        SplitDirection::Y(parts[1].parse().unwrap())
                    }
                })
                .collect::<Vec<SplitDirection>>(),
        )
    };
    let mut fold_map = FoldMap::new(point_data);
    for instruction in split_data {
        match instruction {
            SplitDirection::X(v) => fold_map.fold_h(v),
            SplitDirection::Y(v) => fold_map.fold_v(v),
        }
        break;
    }
    fold_map
        .into_iter()
        .fold(0, |sum, v| if v.1 { sum + 1 } else { sum })
}

pub fn day13_2(file_name: &str) -> impl std::fmt::Debug {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let (point_data, split_data) = {
        let both = fs::read_to_string(input_file)
            .unwrap()
            .split("\n\n")
            .map(|v| v.to_owned())
            .collect::<Vec<_>>();

        (
            both[0]
                .split("\n")
                .map(|line| -> CordPoint {
                    let x_and_y = line
                        .split(",")
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();
                    (x_and_y[0], x_and_y[1])
                })
                .collect::<Vec<_>>(),
            both[1]
                .split("\n")
                .map(|line| {
                    let parts = line.split("=").collect::<Vec<_>>();
                    if parts[0].chars().last().unwrap() == 'x' {
                        SplitDirection::X(parts[1].parse().unwrap())
                    } else {
                        SplitDirection::Y(parts[1].parse().unwrap())
                    }
                })
                .collect::<Vec<SplitDirection>>(),
        )
    };
    let mut fold_map = FoldMap::new(point_data);

    for instruction in split_data {
        match instruction {
            SplitDirection::X(v) => fold_map.fold_h(v),
            SplitDirection::Y(v) => fold_map.fold_v(v),
        }
    }
    fold_map
}

#[cfg(test)]
mod test {
    use crate::assert_eq_dbgfmt;

    use super::*;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_dbgfmt!(17, day13_1("test"));
    }
}
