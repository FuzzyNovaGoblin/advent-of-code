use std::fs;

mod junction_box {
    use std::rc::Rc;

    #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct JunctionBox {
        x: u64,
        y: u64,
        z: u64,
    }

    impl JunctionBox {
        pub fn new(x: u64, y: u64, z: u64) -> Self {
            Self { x, y, z }
        }

        pub fn quick_distance(&self, other: &Self) -> u64 {
            self.x.abs_diff(other.x).pow(3)
                + self.y.abs_diff(other.y).pow(3)
                + self.z.abs_diff(other.z).pow(3)
        }

        pub fn min(rhs: Rc<JunctionBox>, lhs: Rc<JunctionBox>) -> Rc<JunctionBox> {
            if rhs < lhs { rhs.clone() } else { lhs.clone() }
        }

        pub fn max(rhs: Rc<JunctionBox>, lhs: Rc<JunctionBox>) -> Rc<JunctionBox> {
            if rhs > lhs { rhs.clone() } else { lhs.clone() }
        }
    }
}
mod junction_box_set {
    use crate::days::day8::circuit;
    use crate::days::day8::circuit::*;
    use crate::days::day8::junction_box::*;
    use std::collections::HashMap;
    use std::rc::Rc;

    pub struct JunctionBoxSet {
        circuits: Vec<Circuit>,
        boxes: Vec<Rc<JunctionBox>>,
        distances: HashMap<(Rc<JunctionBox>, Rc<JunctionBox>), u64>,
    }

    impl JunctionBoxSet {
        pub fn new<S: AsRef<str>>(source: S) -> Self {
            let (circuits, boxes) = source
                .as_ref()
                .lines()
                .map(|l| {
                    let coords = l
                        .split(',')
                        .map(|pos| pos.trim().parse::<u64>().unwrap())
                        .collect::<Vec<_>>();
                    let b = Rc::new(JunctionBox::new(coords[0], coords[1], coords[2]));
                    (Circuit::from_one(b.clone()), b)
                })
                .collect::<(Vec<_>, Vec<_>)>();

            let distances = boxes
                .iter()
                .flat_map(|first| {
                    boxes
                        .iter()
                        .filter_map(|second| {
                            if first != second {
                                Some((
                                    (
                                        JunctionBox::min(*first, *second),
                                        (JunctionBox::max(*first, *second)),
                                    ),
                                    first.quick_distance(&second),
                                ))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect();

            Self {
                boxes,
                circuits,

                distances,
            }
        }
    }
}

mod circuit {
    use std::rc::Rc;

    use crate::days::day8::junction_box::JunctionBox;

    #[derive(Debug)]
    pub struct Circuit {
        boxes: Vec<Rc<JunctionBox>>,
    }

    impl Circuit {
        pub fn from_one(b: Rc<JunctionBox>) -> Self {
            Self { boxes: vec![b] }
        }
    }
}

pub fn day8_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2025/input/{}",
        env!("ADVENT_OF_CODE_2025"),
        file_name
    );
    let data = fs::read_to_string(input_file);
    todo!()
}

pub fn day8_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2025/input/{}",
        env!("ADVENT_OF_CODE_2025"),
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
        assert_eq_ansval!((), day8_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day8_2("test"));
    }
}
