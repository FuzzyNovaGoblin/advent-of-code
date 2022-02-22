use std::{fs, iter::IntoIterator, ops};

#[derive(Debug)]
struct PointMap {
    points: Vec<Vec<u8>>,
}

struct PointMapIterator {
    x: usize,
    y: usize,
    map: PointMap,
}

impl PointMap {
    fn new(points: Vec<Vec<u8>>) -> Self {
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
                row.push(0);
            }
        }
        println!("{}", std::iter::repeat("*").take(self.points.len()+4).collect::<String>());
        for y in 0..self.points[0].len() {
            print!("**");
            for x in 0..self.points.len() {
                print!("{}", self.points[x][y]);
            }
            println!("**");
        }
        println!("{}", std::iter::repeat("*").take(self.points.len()+4).collect::<String>());
    }
}

impl PointMapIterator {
    fn from(map: PointMap) -> PointMapIterator {
        PointMapIterator { x: 0, y: 0, map }
    }
}

impl Iterator for PointMapIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.y += 1;
        if self.y >= self.map.points[self.x].len() {
            self.y = 0;
            self.x += 1;
        }
        while self.x < self.map.points.len() && self.map.points[self.x].len() < 1 {
            self.x += 1;
        }
        if self.x >= self.map.points.len() {
            None
        } else {
            Some(self.map.points[self.x][self.y])
        }
    }
}

impl IntoIterator for PointMap {
    type Item = u8;

    type IntoIter = PointMapIterator;

    fn into_iter(self) -> Self::IntoIter {
        PointMapIterator::from(self)
    }
}

impl ops::Index<(usize, usize)> for PointMap {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.points[x][y]
    }
}

impl ops::IndexMut<(usize, usize)> for PointMap {
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

pub fn day5_1 (file_name: &str)->  impl crate::AnsType{
	let input_file = format!("{}/aofc_2021/input/{}",env!("ADVENT_OF_CODE_2021"),file_name);
    let data = fs::read_to_string(input_file).unwrap();
    let paths = data
        .split("\n")
        .map(|line| {
            line.split(" -> ")
                .map(|half_line| {
                    half_line
                        .split(",")
                        .filter_map(|num_str| num_str.parse::<u32>().ok())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let paths = paths
        .into_iter()
        .filter(|line| line[0][0] == line[1][0] || line[0][1] == line[1][1])
        .collect::<Vec<_>>();

    let mut points_map = PointMap::new(vec![]);
    for line in paths {

        if line[0][0] == line[1][0] {
            let min = std::cmp::min(line[0][1], line[1][1]);
            let max = std::cmp::max(line[0][1], line[1][1]);
            for y in min..=max {
                points_map[(line[0][0] as usize, y as usize)] += 1;
            }
        } else if line[0][1] == line[1][1] {
            let min = std::cmp::min(line[0][0], line[1][0]);
            let max = std::cmp::max(line[0][0], line[1][0]);
            for x in min..=max {
                points_map[(x as usize, line[0][1] as usize)] += 1;
            }
        } else {
            dbg!("here3");
        }
        // points_map.print_map();
    }

    points_map
        .into_iter()
        .fold(0, |acc, val| if val >= 2 { acc + 1 } else { acc })
}

pub fn day5_2 (file_name: &str)->  impl crate::AnsType{
	let input_file = format!("{}/aofc_2021/input/{}",env!("ADVENT_OF_CODE_2021"),file_name);
    let data = fs::read_to_string(input_file).unwrap();
    let paths = data
        .split("\n")
        .map(|line| {
            line.split(" -> ")
                .map(|half_line| {
                    half_line
                        .split(",")
                        .filter_map(|num_str| num_str.parse::<u32>().ok())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();


    let mut points_map = PointMap::new(vec![]);
    for line in paths {

        if line[0][0] == line[1][0] {
            let min = std::cmp::min(line[0][1], line[1][1]);
            let max = std::cmp::max(line[0][1], line[1][1]);
            for y in min..=max {
                points_map[(line[0][0] as usize, y as usize)] += 1;
            }
        } else if line[0][1] == line[1][1] {
            let min = std::cmp::min(line[0][0], line[1][0]);
            let max = std::cmp::max(line[0][0], line[1][0]);
            for x in min..=max {
                points_map[(x as usize, line[0][1] as usize)] += 1;
            }
        } else {
            let mut point_x = line[0][0];
            let mut point_y = line[0][1];
            while point_x != line[1][0] {
                points_map[(point_x as usize,point_y as usize)]+=1;
                if line[1][0] > point_x{
                    point_x+=1;
                }
                else{
                    point_x-=1;
                }
                if line[1][1] > point_y{
                    point_y+=1;
                }
                else{
                    point_y-=1;
                }
            }
                points_map[(point_x as usize,point_y as usize)]+=1;
        }
        // points_map.print_map();
    }

    points_map
        .into_iter()
        .fold(0, |acc, val| if val >= 2 { acc + 1 } else { acc })
}
