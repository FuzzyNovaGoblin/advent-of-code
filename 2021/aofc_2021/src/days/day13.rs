use std::fs;

use crate::{
    days::day13::fold_map::{FoldMap, MapFold, SplitDirection},
    point_map::CordPointTuple,
    AnsType,
};

mod fold_map {
    use std::collections::HashMap;
    use std::{fmt, ops};

    use crate::point_map::CordPointTuple;
    use crate::AnsType;

    #[derive(Default)]
    pub struct FoldMap {
        points: HashMap<CordPointTuple, bool>,
        lowest_point: CordPointTuple,
        greatest_point: CordPointTuple,
    }

    impl IntoIterator for FoldMap {
        type Item = (CordPointTuple, bool);

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
                writeln!(f)?;
            }
            Ok(())
        }
    }

    impl AnsType for FoldMap {
        fn value(&self) -> String {
            format!("{:?}", self)
        }
    }

    impl FoldMap {
        pub fn new(new_points: Vec<CordPointTuple>) -> Self {
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

    impl ops::Index<CordPointTuple> for FoldMap {
        type Output = bool;

        fn index(&self, index: CordPointTuple) -> &Self::Output {
            &self.points[&index]
        }
    }
    impl ops::IndexMut<CordPointTuple> for FoldMap {
        fn index_mut(&mut self, index: CordPointTuple) -> &mut Self::Output {
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

pub fn day13_1(file_name: &str) -> impl AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        std::env::var("ADVENT_OF_CODE_2021").unwrap(),
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
                .split('\n')
                .map(|line| -> CordPointTuple {
                    let x_and_y = line
                        .split(',')
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();
                    (x_and_y[0], x_and_y[1])
                })
                .collect::<Vec<_>>(),
            both[1]
                .split('\n')
                .map(|line| {
                    let parts = line.split('=').collect::<Vec<_>>();
                    if parts[0].ends_with('x') {
                        SplitDirection::X(parts[1].parse().unwrap())
                    } else {
                        SplitDirection::Y(parts[1].parse().unwrap())
                    }
                })
                .collect::<Vec<SplitDirection>>(),
        )
    };
    let mut fold_map = FoldMap::new(point_data);
    if let Some(instruction) = split_data.into_iter().next() {
        match instruction {
            SplitDirection::X(v) => fold_map.fold_h(v),
            SplitDirection::Y(v) => fold_map.fold_v(v),
        }
    }
    fold_map
        .into_iter()
        .fold(0, |sum, v| if v.1 { sum + 1 } else { sum })
}

pub fn day13_2(file_name: &str) -> impl AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        std::env::var("ADVENT_OF_CODE_2021").unwrap(),
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
                .split('\n')
                .map(|line| -> CordPointTuple {
                    let x_and_y = line
                        .split(',')
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();
                    (x_and_y[0], x_and_y[1])
                })
                .collect::<Vec<_>>(),
            both[1]
                .split('\n')
                .map(|line| {
                    let parts = line.split('=').collect::<Vec<_>>();
                    if parts[0].ends_with('x') {
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
