use std::{collections::HashSet, fmt::Display, fs};

use self::rotation::{generate_rotation_set, Rotation};

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

    fn manhattan_distance(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn get_with_rotaion(&self, rotation: &rotation::Rotation) -> Point {
        Point::new(
            rotation.translate_x(self) * rotation.x.direction.as_int(),
            rotation.translate_y(self) * rotation.y.direction.as_int(),
            rotation.translate_z(self) * rotation.z.direction.as_int(),
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
    name: String,
    points: Vec<Point>,
}

impl Scanner {
    fn get_all_with_rotaion(&self, rotation: &rotation::Rotation) -> Vec<Point> {
        self.points
            .iter()
            .map(|p| p.get_with_rotaion(rotation))
            .collect()
    }

    fn build_from_str(str_data: &str) -> Scanner {
        let mut lines = str_data.split('\n');
        let name = lines.next().unwrap().to_owned();

        Scanner {
            name,
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

    fn can_fit(
        &self,
        new_scanner: &Scanner,
        incomming_rotation: &Rotation,
        rotation_set: &HashSet<Rotation>,
    ) -> Option<(rotation::Rotation, Point)> {
        // for each of the original points being the aligning point
        for base_point in self.get_all_with_rotaion(incomming_rotation).iter() {
            // for every posible rotation
            for r in rotation_set {
                let new_points = new_scanner.get_all_with_rotaion(r);
                // for each of the sub points being the aligning point
                for offset in new_points.iter().map(|p| base_point - p) {
                    let mut valid_count = 0;
                    for (enum_index, point) in self
                        .get_all_with_rotaion(incomming_rotation)
                        .iter()
                        .enumerate()
                    {
                        for second_point in new_points.iter() {
                            if &(second_point + &offset) == point {
                                valid_count += 1;
                                if valid_count >= 12 {
                                    return Some((r.clone(), offset));
                                }
                            }
                        }
                        if (25 - enum_index + valid_count) < (12) {
                            break;
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
    scanners: Vec<(Scanner, rotation::Rotation, Point)>,
    rotation_set: HashSet<Rotation>,
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
            rotation_set: generate_rotation_set(),
        }
    }

    fn fits_in_map(&self, new_scanner: &Scanner) -> Option<(rotation::Rotation, Point)> {
        for (scanner, rotation, offset) in &self.scanners {
            if let Some((sub_rot, sub_offset)) =
                scanner.can_fit(new_scanner, rotation, &self.rotation_set)
            {
                return Some((sub_rot, sub_offset + offset));
            }
        }
        None
    }
}

pub fn day19_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        std::env::var("ADVENT_OF_CODE_2021").unwrap(),
        file_name
    );

    let data = fs::read_to_string(input_file).unwrap();
    let mut scanners = data
        .split("\n\n")
        .map( Scanner::build_from_str)
        .collect::<Vec<_>>();

    let mut probe_map = ProbeMap::new(scanners.remove(0));

    'until_no_scanners: loop {
        if scanners.is_empty() {
            break;
        }
        for i in (0..scanners.len()).rev() {
            if let Some((rot, offset)) = probe_map.fits_in_map(&scanners[i]) {
                let scanner = scanners.remove(i);
                let mut points = scanner
                    .points
                    .iter()
                    .map(|p| p.get_with_rotaion(&rot) + &offset)
                    .collect::<Vec<_>>();
                points.sort();
                for point in points {
                    probe_map.points.insert(point.clone());
                }

                probe_map.scanners.push((scanner, rot, offset));
                continue 'until_no_scanners;
            }
        }
        unreachable!();
    }

    let mut points = probe_map.points.iter().collect::<Vec<_>>();
    points.sort();

    probe_map.points.len()
}

pub fn day19_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        std::env::var("ADVENT_OF_CODE_2021").unwrap(),
        file_name
    );

    let data = fs::read_to_string(input_file).unwrap();
    let mut scanners = data
        .split("\n\n")
        .map(Scanner::build_from_str)
        .collect::<Vec<_>>();

    let mut probe_map = ProbeMap::new(scanners.remove(0));

    'until_no_scanners: loop {
        if scanners.is_empty(){
            break;
        }

        for i in (0..scanners.len()).rev() {
            if let Some((rot, offset)) = probe_map.fits_in_map(&scanners[i]) {
                let scanner = scanners.remove(i);
                let mut points = scanner
                    .points
                    .iter()
                    .map(|p| p.get_with_rotaion(&rot) + &offset)
                    .collect::<Vec<_>>();
                points.sort();
                for point in points {
                    probe_map.points.insert(point.clone());
                }
                probe_map.scanners.push((scanner, rot, offset));
                println!("found {} of ", probe_map.scanners.len());

                continue 'until_no_scanners;
            }
        }
        unreachable!();
    }

    let mut points = probe_map.scanners.iter().map(|s| &s.2).collect::<Vec<_>>();
    points.sort();
    eprintln!("\n\n",);
    let mut max_distance = 0;
    for (e1, p1) in points.iter().enumerate() {
        for (e2, p2) in points.iter().enumerate() {
            if e1 == e2 {
                continue;
            }

            max_distance = p1.manhattan_distance(p2).max(max_distance);
        }
    }

    max_distance
}

mod rotation {

    use std::collections::HashSet;

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