use crate::days::day7::manifold_map::*;
use std::fs;

mod map_point {
    use std::fmt::Display;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum MapPoint {
        Blank,
        Source,
        Splitter,
        Beam,
        BeamPath(u64),
    }

    impl MapPoint {
        pub fn count(&self) -> u64 {
            match self {
                Self::BeamPath(v) => *v,
                _ => 0,
            }
        }
    }

    impl Display for MapPoint {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let c = match self {
                MapPoint::Blank => ".",
                MapPoint::Source => "S",
                MapPoint::Splitter => "^",
                MapPoint::Beam => "|",
                MapPoint::BeamPath(v) => &format!("{}", v),
            };
            write!(f, "{}", c)
        }
    }

    impl From<char> for MapPoint {
        fn from(value: char) -> Self {
            match value {
                'S' => MapPoint::Source,
                '^' => MapPoint::Splitter,
                _ => Self::Blank,
            }
        }
    }
}

mod manifold_map {
    use crate::days::day7::map_point::MapPoint;
    use crate::days::day7::map_point::MapPoint::*;
    use std::{
        fmt::{Debug, Display},
        ops,
    };

    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct ManifoldMap {
        points: Vec<Vec<MapPoint>>,
        source: usize,
    }

    impl ManifoldMap {
        pub fn _width(&self) -> usize {
            self.points[0].len()
        }

        pub fn height(&self) -> usize {
            self.points.len()
        }

        pub fn new<S: AsRef<str>>(source: S) -> Self {
            let points = source
                .as_ref()
                .lines()
                .map(|l| l.chars().map(MapPoint::from).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            Self {
                source: points[0]
                    .iter()
                    .position(|&p| p == MapPoint::Source)
                    .unwrap(),
                points,
            }
        }
        pub fn simulate_beam(&self) -> u64 {
            let mut count = 0;
            let mut beam_positions = vec![self.source];
            for i in 1..self.height() {
                let mut next_layer = self.points[i].clone();
                for &bp in beam_positions.iter() {
                    match next_layer[bp] {
                        Blank => next_layer[bp] = Beam,

                        Splitter => {
                            let mut splits = false;
                            if next_layer[bp - 1] != Beam {
                                next_layer[bp - 1] = Beam;
                                splits = true;
                            }
                            if next_layer[bp + 1] != Beam {
                                next_layer[bp + 1] = Beam;
                                splits = true;
                            }
                            if splits {
                                count += 1
                            }
                        }

                        BeamPath(_) | Beam => (),
                        Source => unreachable!(),
                    }
                }
                beam_positions = next_layer
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &mp)| if mp == Beam { Some(i) } else { None })
                    .collect();
            }

            count
        }

        pub fn simulate_quantum_beam(&self) -> Vec<u64> {
            let mut count_map = self.points.clone();
            count_map[0][self.source] = BeamPath(1);

            let mut beam_positions = vec![self.source];
            for i in 1..self.height() {
                for &bp in beam_positions.iter() {
                    match count_map[i][bp] {
                        Blank => count_map[i][bp] = BeamPath(count_map[i - 1][bp].count()),
                        Splitter => {
                            count_map[i][bp - 1] = match count_map[i][bp - 1] {
                                Blank => BeamPath(count_map[i - 1][bp].count()),
                                BeamPath(c) => BeamPath(c + count_map[i - 1][bp].count()),
                                _ => unreachable!(),
                            };
                            count_map[i][bp + 1] = match count_map[i][bp + 1] {
                                Blank => BeamPath(count_map[i - 1][bp].count()),
                                BeamPath(c) => BeamPath(c + count_map[i - 1][bp].count()),
                                _ => unreachable!(),
                            };
                        }
                        BeamPath(c) => {
                            count_map[i][bp] = BeamPath(c + count_map[i - 1][bp].count())
                        }
                        _ => unreachable!(),
                    }
                }
                beam_positions = count_map[i]
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &mp)| {
                        if matches!(mp, BeamPath(_)) {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect();
            }

            count_map
                .last()
                .unwrap()
                .iter()
                .map(MapPoint::count)
                .collect()
        }
    }

    impl Debug for ManifoldMap {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "source:{:?}", self.source)?;
            writeln!(f, "points:\n{}", self)
        }
    }

    impl Display for ManifoldMap {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                self.points
                    .iter()
                    .map(|l| l.iter().map(|c| format!("{}", c)).collect::<String>() + "\n")
                    .collect::<String>()
            )
        }
    }

    impl ops::Index<usize> for ManifoldMap {
        type Output = Vec<MapPoint>;

        fn index(&self, index: usize) -> &Self::Output {
            &self.points[index]
        }
    }
    impl ops::IndexMut<usize> for ManifoldMap {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.points[index]
        }
    }
}

pub fn day7_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2025/input/{}",
        env!("ADVENT_OF_CODE_2025"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();

    let map = ManifoldMap::new(data);
    map.simulate_beam()
}

pub fn day7_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2025/input/{}",
        env!("ADVENT_OF_CODE_2025"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();

    let map = ManifoldMap::new(data);
    map.simulate_quantum_beam().iter().sum::<u64>()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!(1711, day7_1("day7"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!(36706966158365_u64, day7_2("day7"));
    }
}
