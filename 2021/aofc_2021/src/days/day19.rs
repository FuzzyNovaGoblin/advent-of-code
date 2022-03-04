use std::{cell::RefCell, collections::HashSet, fmt::Display, fs};

use self::rotation::Rotation;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn get_with_rotaion(&self, rotation: &rotation::Rotation) -> Point {
        Point::new(
            rotation.translate_x(&self) * rotation.x.direction.as_int(),
            rotation.translate_y(&self) * rotation.y.direction.as_int(),
            rotation.translate_z(&self) * rotation.z.direction.as_int(),
        )
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl std::ops::Add<&Point> for Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl std::ops::Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl std::ops::Add<Point> for &Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Debug, Hash)]
struct Scanner {
    points: Vec<Point>,
}

impl Scanner {
    #[allow(dead_code)]
    fn new(points: Vec<Point>) -> Self {
        Self { points }
    }
    fn get_all_with_rotaion(&self, rotation: &rotation::Rotation) -> Vec<Point> {
        self.points
            .iter()
            .map(|p| p.get_with_rotaion(rotation))
            .collect()
    }
    fn build_from_str(str_data: &str) -> Scanner {
        let mut lines = str_data.split('\n');
        let _name = lines.next().unwrap();

        Scanner {
            points: lines
                .map(|line| {
                    let points: Vec<_> = line.split(',').collect();
                    Point::new(
                        points[0].parse().unwrap(),
                        points[1].parse().unwrap(),
                        points[2].parse().unwrap(),
                    )
                })
                .collect(),
        }
    }

    fn can_fit(&self, new_scanner: &Scanner, incomming_rotation: &Rotation, incomming_offset: &Point) -> Option<(rotation::Rotation, Point)> {
        // for base_point in self.points.iter() {
        for base_point in self.get_all_with_rotaion(incomming_rotation).iter() {
            for r in rotation::generate_rotation_set() {
                let new_points = new_scanner.get_all_with_rotaion(&r);
                for (_first_point, offset) in new_points.iter().map(|p| (p, base_point - p )) {
                    let mut valid_count = 0;
                    for point in self.get_all_with_rotaion(incomming_rotation).iter() {
                        'each_point_against_original: for second_point in new_points.iter() {
                            if &(second_point + &offset) == point {
                                valid_count += 1;
                                if valid_count >= 12 {
                                    return Some((r, offset));
                                }
                                break 'each_point_against_original;
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

#[derive(Debug)]
struct ProbeMap {
    points: HashSet<Point>,
    scanners: Vec<(Scanner, rotation::Rotation, Point /* offset */)>,
}

impl ProbeMap {
    fn new(first_scanner: Scanner) -> Self {
        Self {
            points: first_scanner.points.iter().cloned().collect(),
            scanners: vec![(
                first_scanner,
                rotation::Rotation::default(),
                Point::new(0, 0, 0),
            )],
        }
    }

    // fn fits_in_map(&self, new_scanner: &Scanner) -> Option<(rotation::Rotation, Point)> {
    //     for base_point in self.points.iter() {
    //         for r in rotation::generate_rotation_set() {
    //             let new_points = new_scanner.get_all_with_rotaion(&r);
    //             for (_first_point, offset) in new_points.iter().map(|p| (p, base_point - p)) {
    //                 let mut valid_count = 0;
    //                 for point in self.points.iter() {
    //                     'each_point_against_original: for second_point in new_points.iter() {
    //                         if &(second_point + &offset) == point {
    //                             valid_count += 1;

    //                             break 'each_point_against_original;
    //                         }
    //                     }
    //                 }
    //                 if valid_count >= 12 {
    //                     return Some((r, offset));
    //                 }
    //             }
    //         }
    //     }
    //     None
    // }

    // fn fits_in_map(&self, new_scanner: &Scanner) -> Option<(Scanner, rotation::Rotation, Point)> {
    fn fits_in_map(
        &self,
        new_scanner: &Scanner,
    ) -> Option<(/* Scanner, */ rotation::Rotation, Point)> {
        for (scanner, rotation, offset) in &self.scanners {
            // dbg!(count);
            if let Some((sub_rot, sub_offset)) = scanner.can_fit(new_scanner, &rotation, &offset) {
                println!("sub_offset: {}\toffset: {}", sub_offset, offset);
                return Some((   sub_rot , sub_offset+offset ));
            }
        }
        None
    }
}

pub fn day19_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );

    let data = fs::read_to_string(input_file).unwrap();
    let mut scanners = data
        .split("\n\n")
        .map(|sdata| Scanner::build_from_str(sdata))
        .collect::<Vec<_>>();

    let mut probe_map = ProbeMap::new(scanners.remove(0));

    'until_no_scanners: loop {
        if scanners.len() == 0 {
            break;
        }
        for i in (0..scanners.len()).rev() {
            if let Some((/* scann, */ rot, offset)) = probe_map.fits_in_map(&scanners[i]) {
                let scanner = scanners.remove(i);
                // let scanner = scann;
                let mut points = scanner
                    .points
                    .iter()
                    .map(|p| p.get_with_rotaion(&rot) + &offset)
                    .collect::<Vec<_>>();
                points.sort();
                for point in points {
                    probe_map.points.insert(point.clone());
                }
                println!(
                    "added scanner {:?}\noffset: {:?}\nrotation:{:?}",
                    scanner, offset, rot
                );

                probe_map.scanners.push((scanner, rot, offset));
                continue 'until_no_scanners;
            }
        }
        unreachable!();
    }

    let mut points = probe_map.points.iter().collect::<Vec<_>>();
    points.sort();
    println!("\n\n",);
    for p in points {
        println!("{}", p);
    }

    probe_map.points.len()
    // let rotations = rotation::generate_rotation_set();

    // let v = rotations
    //     .iter()
    //     .map(|r| {
    //         points
    //             .iter()
    //             .map(|p| p.get_with_rotaion(&r))
    //             .collect::<Vec<Point>>()
    //     })
    //     .collect::<Vec<Vec<Point>>>();

    // let example_points = vec![
    //     vec![
    //         Point::new(-1, -1, 1),
    //         Point::new(-2, -2, 2),
    //         Point::new(-3, -3, 3),
    //         Point::new(-2, -3, 1),
    //         Point::new(5, 6, -4),
    //         Point::new(8, 0, 7),
    //     ],
    //     vec![
    //         Point::new(1, -1, 1),
    //         Point::new(2, -2, 2),
    //         Point::new(3, -3, 3),
    //         Point::new(2, -1, 3),
    //         Point::new(-5, 4, -6),
    //         Point::new(-8, -7, 0),
    //     ],
    //     vec![
    //         Point::new(-1, -1, -1),
    //         Point::new(-2, -2, -2),
    //         Point::new(-3, -3, -3),
    //         Point::new(-1, -3, -2),
    //         Point::new(4, 6, 5),
    //         Point::new(-7, 0, 8),
    //     ],
    //     vec![
    //         Point::new(1, 1, -1),
    //         Point::new(2, 2, -2),
    //         Point::new(3, 3, -3),
    //         Point::new(1, 3, -2),
    //         Point::new(-4, -6, 5),
    //         Point::new(7, 0, 8),
    //     ],
    //     vec![
    //         Point::new(1, 1, 1),
    //         Point::new(2, 2, 2),
    //         Point::new(3, 3, 3),
    //         Point::new(3, 1, 2),
    //         Point::new(-6, -4, -5),
    //         Point::new(0, 7, -8),
    //     ],
    // ];

    // for ep in example_points {
    //     if v.contains(&ep) {
    //         println!("valid");
    //     } else {
    //         panic!("not valid");
    //     }
    // }

    // for r in rotations {
    //     // let mut new_points = points.clone();

    //     for p in points.iter() {
    //         println!("{}", p.get_with_rotaion(r.clone()));
    //     }
    //     println!();
    // }
}

pub fn day19_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let _data = fs::read_to_string(input_file);
    todo!()
}

mod rotation {

    use std::{borrow::Borrow, collections::HashSet};

    use Polarity::*;
    use RotationAxis::*;

    use super::Point;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum RotationAxis {
        X,
        Y,
        Z,
    }

    impl RotationAxis {
        fn get_other_directions(&self) -> (RotationAxis, RotationAxis) {
            match self {
                X => (Y, Z),
                Y => (Z, X),
                Z => (X, Y),
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Polarity {
        Positive,
        Negetive,
    }

    impl Polarity {
        fn oposite(&self) -> Self {
            match self {
                Positive => Negetive,
                Negetive => Positive,
            }
        }
        pub fn as_int(&self) -> i64 {
            match self {
                Positive => 1,
                Negetive => -1,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct RotationPair {
        pub axis: RotationAxis,
        pub direction: Polarity,
    }

    impl RotationPair {
        fn new(axis: RotationAxis, direction: Polarity) -> Self {
            Self { axis, direction }
        }
    }

    #[derive(Debug, Eq, Clone)]
    pub struct Rotation {
        pub x: RotationPair,
        pub y: RotationPair,
        pub z: RotationPair,
        history: Vec<RotationAxis>,
    }

    impl std::hash::Hash for Rotation {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.x.hash(state);
            self.y.hash(state);
            self.z.hash(state);
        }
    }

    impl PartialEq for Rotation {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y && self.z == other.z
        }
    }

    impl Default for Rotation {
        fn default() -> Self {
            Self {
                x: RotationPair::new(X, Positive),
                y: RotationPair::new(Y, Positive),
                z: RotationPair::new(Z, Positive),
                history: Vec::new(),
            }
        }
    }

    impl Rotation {
        fn get_tmp_rotation_type_from_axis(&mut self, axis: RotationAxis) -> &mut RotationPair {
            match axis {
                X => &mut self.x,
                Y => &mut self.y,
                Z => &mut self.z,
            }
        }

        fn rotation_type_from_axis(&self, axis: RotationAxis) -> RotationPair {
            match axis {
                X => self.x.clone(),
                Y => self.y.clone(),
                Z => self.z.clone(),
            }
        }

        pub fn translate_x(&self, point: &Point) -> i64 {
            match self.x.axis {
                X => point.x,
                Y => point.y,
                Z => point.z,
            }
        }
        pub fn translate_y(&self, point: &Point) -> i64 {
            match self.y.axis {
                X => point.x,
                Y => point.y,
                Z => point.z,
            }
        }
        pub fn translate_z(&self, point: &Point) -> i64 {
            match self.z.axis {
                X => point.x,
                Y => point.y,
                Z => point.z,
            }
        }

        pub fn rotate(&mut self, axis: RotationAxis) {
            {
                self.history.push(axis);
                let (axis1, axis2) = axis.get_other_directions();
                let (original_rotation_val1, original_rotation_val2) = (
                    self.get_tmp_rotation_type_from_axis(axis1).clone(),
                    self.get_tmp_rotation_type_from_axis(axis2).clone(),
                );
                self.get_tmp_rotation_type_from_axis(axis1).axis = original_rotation_val2.axis;
                self.get_tmp_rotation_type_from_axis(axis2).axis = original_rotation_val1.axis;
                self.get_tmp_rotation_type_from_axis(axis1).direction =
                    original_rotation_val2.direction.oposite();

                self.get_tmp_rotation_type_from_axis(axis2).direction =
                    original_rotation_val1.direction;
            }
        }
    }

    impl AsRef<Rotation> for Rotation {
        fn as_ref(&self) -> &Rotation {
            &self
        }
    }

    impl<T: AsRef<Rotation>> std::ops::Add<T> for Rotation {
        type Output = Rotation;

        fn add(self, rhs: T) -> Self::Output {
            &self + rhs
        }
    }
    impl<T: AsRef<Rotation>> std::ops::Add<T> for &Rotation {
        type Output = Rotation;

        fn add(self, rhs: T) -> Self::Output {
            let right = rhs.as_ref();
            let mut ret_val = self.clone();
            for thing in right.history.iter(){
                ret_val.rotate(thing.clone());
            }
            ret_val
        }
    }
    pub fn generate_rotation_set() -> HashSet<Rotation> {
        let mut rot_set = HashSet::<Rotation>::new();

        for x in 0..=3 {
            let mut rot = Rotation::default();
            for _ in 0..x {
                rot.rotate(X)
            }
            for y in 0..=3 {
                for _ in 0..y {
                    rot.rotate(Y)
                }
                for z in 0..=3 {
                    for _ in 0..z {
                        rot.rotate(Z)
                    }
                    rot_set.insert(rot.clone());
                }
            }
        }
        rot_set
    }
}

#[cfg(test)]
mod test {
    use super::rotation::{Rotation, RotationAxis::*};
    use super::{day19_1, day19_2};
    use crate::assert_eq_ansval;
    use crate::days::day19::rotation::generate_rotation_set;

    #[test]
    fn rotation_test() {
        let mut goal = Rotation::default();
        let mut r1 = Rotation::default();
        let mut r2 = Rotation::default();
        let mut r3 = Rotation::default();
        let mut r4 = Rotation::default();
        r1.rotate(X);
        r2.rotate(Y);
        goal.rotate(X);
        goal.rotate(Y);

        dbg!(&goal);
        assert_eq!(goal, &r1 + &r2 /* + r4 */);
        goal.rotate(Y);
        // goal.rotate(Z);
        r3.rotate(Y);
        dbg!(&r3);
        r4.rotate(Z);
        assert_eq!(goal, r1 + r2 + r3 /* + r4 */);
    }

    #[test]
    fn posible_rotations() {
        assert_eq!(24, generate_rotation_set().len());
    }

    #[test]
    fn t1() {
        assert_eq_ansval!(79, day19_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day19_2("test"));
    }
}
