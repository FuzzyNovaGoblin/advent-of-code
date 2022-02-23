use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    fs, ops,
};

#[derive(Debug, Default)]
struct PointMap<T> {
    points: Vec<Vec<T>>,
}

impl<T> PointMap<T>
where
    T: Display + Default + PartialEq<u32> + PartialOrd,
{
    #[allow(dead_code)]
    fn new(points: Vec<Vec<T>>) -> Self {
        Self { points }
    }

    #[allow(dead_code)]
    fn print_map(&mut self) {
        let mut max_width = 0;
        for row in self.points.iter() {
            if row.len() > max_width {
                max_width = row.len();
            }
        }
        for row in self.points.iter_mut() {
            while row.len() < max_width {
                row.push(Default::default());
            }
        }
        println!(
            "{}",
            "*".repeat(self.points.len() + 4)
        );
        for y in 0..self.points[0].len() {
            print!("**");
            for x in 0..self.points.len() {
                print!("{}", self.points[x][y]);
            }
            println!("**");
        }
        println!(
            "{}",
            "*".repeat(self.points.len() + 4)
        );
    }

    #[allow(dead_code)]
    fn print_around_point(&mut self, x: usize, y: usize) {
        // first layer
        if y > 0 {
            if x > 0 {
                print!("{}", self[(x - 1, y - 1)]);
            } else {
                print!("*");
            }
            print!("{}", self[(x, y - 1)]);
            if x < self.points.len() - 1 {
                print!("{}", self[(x + 1, y - 1)]);
            } else {
                print!("*");
            }
        } else {
            print!("***");
        }

        // second layer
        println!();
        if x > 0 {
            print!("{}", self[(x - 1, y)]);
        } else {
            print!("*");
        }
        print!("{}", self[(x, y)]);
        if x < self.points.len() - 1 {
            print!("{}", self[(x + 1, y)]);
        } else {
            print!("*");
        }
        println!();

        // third layer
        if y < self.points[0].len() - 1 {
            if x > 0 {
                print!("{}", self[(x - 1, y + 1)]);
            } else {
                print!("*");
            }
            print!("{}", self[(x, y + 1)]);
            if x < self.points.len() - 1 {
                print!("{}", self[(x + 1, y + 1)]);
            } else {
                print!("*");
            }
        } else {
            print!("***");
        }
        println!();
    }

    fn get_basin(
        &self,
        mut points_set: HashSet<(usize, usize)>,
        point: (usize, usize),
    ) -> HashSet<(usize, usize)> {
        if self[point] == 9{
            return  points_set;
        }
        points_set.insert(point);
        let (x, y) = point;

        if x > 0 && !points_set.contains(&(x - 1, y)) {
            points_set = self.get_basin(points_set, (x - 1, y));
        }
        if x < self.points.len() - 1 && !points_set.contains(&(x + 1, y)) {
            points_set = self.get_basin(points_set, (x + 1, y));
        }
        if y > 0 && !points_set.contains(&(x, y - 1)) {
            points_set = self.get_basin(points_set, (x, y - 1));
        }
        if y < self.points[0].len() - 1 && !points_set.contains(&(x, y + 1)) {
            points_set = self.get_basin(points_set, (x, y + 1));
        }
        points_set
    }
}

impl<T> ops::Index<(usize, usize)> for PointMap<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.points[x][y]
    }
}

impl<T> ops::IndexMut<(usize, usize)> for PointMap<T>
where
    T: Display + Default,
{
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        while x >= self.points.len() {
            self.points.push(Default::default());
        }
        while y >= self.points[x].len() {
            self.points[x].push(Default::default());
        }

        &mut self.points[x][y]
    }
}

pub fn day9_1 (file_name: &str)->  impl crate::AnsType{
	let input_file = format!("{}/aofc_2021/input/{}",env!("ADVENT_OF_CODE_2021"),file_name);
    let _data = fs::read_to_string(input_file).unwrap();
    let mut point_map = PointMap::default();
    for (y, line) in _data.split('\n').enumerate() {
        for (x, val) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            point_map[(x, y)] = val;
        }
    }
    let mut low_points = vec![];
    for y in 0..point_map.points[0].len() {
        for x in 0..point_map.points.len() {
            if x > 0 && point_map[(x - 1, y)] <= point_map[(x, y)] {
                continue;
            }
            if x < point_map.points.len() - 1 && point_map[(x + 1, y)] <= point_map[(x, y)] {
                continue;
            }
            if y > 0 && point_map[(x, y - 1)] <= point_map[(x, y)] {
                continue;
            }
            if y < point_map.points[0].len() - 1 && point_map[(x, y + 1)] <= point_map[(x, y)] {
                continue;
            }

            low_points.push(point_map[(x, y)]);
        }
    }
    low_points
        .into_iter()
        .fold(0, |sum_val, v| sum_val + (v + 1))
}

pub fn day9_2 (file_name: &str)->  impl crate::AnsType{
	let input_file = format!("{}/aofc_2021/input/{}",env!("ADVENT_OF_CODE_2021"),file_name);
    let _data = fs::read_to_string(input_file).unwrap();
    let mut point_map = PointMap::default();
    for (y, line) in _data.split('\n').enumerate() {
        for (x, val) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            point_map[(x, y)] = val;
        }
    }
    let mut low_points = vec![];
    for y in 0..point_map.points[0].len() {
        for x in 0..point_map.points.len() {
            if x > 0 && point_map[(x - 1, y)] <= point_map[(x, y)] {
                continue;
            }
            if x < point_map.points.len() - 1 && point_map[(x + 1, y)] <= point_map[(x, y)] {
                continue;
            }
            if y > 0 && point_map[(x, y - 1)] <= point_map[(x, y)] {
                continue;
            }
            if y < point_map.points[0].len() - 1 && point_map[(x, y + 1)] <= point_map[(x, y)] {
                continue;
            }
            low_points.push((x, y));
        }
    }
    let mut basins: Vec<_> = vec![];
    for point in low_points {
        basins.push(point_map.get_basin(Default::default(), point).len());
    }
    basins.sort_unstable();
    basins.reverse();
    basins.iter().take(3).product::<usize>()
}
