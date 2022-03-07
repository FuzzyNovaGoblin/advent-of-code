use crate::point_map::CordPoint;
use std::{
    collections::HashMap,
    default::default,
    fs,
    ops::{Index, IndexMut},
};

#[derive(Debug)]
struct PointHashMap {
    map: HashMap<CordPoint, char>,
    default_point: char,
}

impl Default for PointHashMap {
    fn default() -> Self {
        Self {
            map: Default::default(),
            default_point: '.',
        }
    }
}

impl Index<CordPoint> for PointHashMap {
    type Output = char;

    fn index(&self, index: CordPoint) -> &Self::Output {
        if self.map.contains_key(&index) {
            self.map.get(&index).unwrap()
        } else {
            &self.default_point
        }
    }
}
impl IndexMut<CordPoint> for PointHashMap {
    fn index_mut(&mut self, index: CordPoint) -> &mut Self::Output {
        if !self.map.contains_key(&index) {
            self.map.insert(index.clone(), '.');
        }
        self.map.get_mut(&index).unwrap()
    }
}

impl PointHashMap {
    fn get_surounding_points(&self, point: &CordPoint) -> Vec<char> {
        let mut ret_val = Vec::new();
        for y in point.y - 1..=point.y + 1 {
            for x in point.x - 1..=point.x + 1 {
                ret_val.push(self[CordPoint::new(x, y)]);
            }
        }
        ret_val
    }
    fn get_surounding_points_num(
        &self,
        /*         save_map: HashMap<Vec<char>, usize>,
         */ point: &CordPoint,
    ) -> usize {
        let chars = self.get_surounding_points(point);

        // loop {
        //     if save_map.contains_key(&chars) {
        //         break;
        //     }

        let mut ret_val = 0;
        for c in chars {
            if c == '#' {
                ret_val += 1;
            }
            ret_val <<= 1;
        }
        ret_val >>= 1;

        // break;
        // }

        ret_val
    }

    fn print_map(&self) {
        let (min, max) = self.get_dimentions();
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                print!("{}", self[CordPoint::new(x, y)])
            }
            println!();
        }
    }
    fn get_dimentions(&self) -> (CordPoint, CordPoint) {
        let (mut max_x, mut min_x, mut max_y, mut min_y) = (0, 0, 0, 0);
        for key in self.map.keys() {
            min_x = min_x.min(key.x);
            max_x = max_x.max(key.x);
            min_y = min_y.min(key.y);
            max_y = max_y.max(key.y);
        }
        (CordPoint::new(min_x, min_y), CordPoint::new(max_x, max_y))
    }
}

pub fn day20_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let data = fs::read_to_string(input_file)
        .unwrap()
        .split("\n\n")
        .map(str::to_owned)
        .collect::<Vec<String>>();
    let image_enhancement_algorithm = data[0].chars().collect::<Vec<char>>();
    let mut input_image: PointHashMap = PointHashMap::default();

    for (y, line) in data[1].split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            input_image[CordPoint::new(x as i128, y as i128)] = c;
        }
    }
    for _ in 0..2 {
        let (min, max) = input_image.get_dimentions();
        let mut new_image: PointHashMap = default();
        for y in min.y - 1..=max.y + 1 {
            for x in min.x - 1..=max.x + 1 {
                new_image[CordPoint::new(x, y)] = image_enhancement_algorithm
                    [input_image.get_surounding_points_num(&CordPoint::new(x, y))];
            }
        }
        input_image = new_image;
        input_image.default_point = if input_image.default_point == '.' {
            image_enhancement_algorithm[0]
        } else {
            image_enhancement_algorithm[1]
        }
    }

    input_image.map.into_iter().fold(
        0,
        |f_val, (_, n_val)| if n_val == '#' { f_val + 1 } else { f_val },
    )
}

pub fn day20_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let data = fs::read_to_string(input_file)
        .unwrap()
        .split("\n\n")
        .map(str::to_owned)
        .collect::<Vec<String>>();
    let image_enhancement_algorithm = data[0].chars().collect::<Vec<char>>();
    let mut input_image: PointHashMap = PointHashMap::default();

    for (y, line) in data[1].split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            input_image[CordPoint::new(x as i128, y as i128)] = c;
        }
    }
    for _ in 0..50 {
        let (min, max) = input_image.get_dimentions();
        let mut new_image: PointHashMap = default();
        for y in min.y - 1..=max.y + 1 {
            for x in min.x - 1..=max.x + 1 {
                new_image[CordPoint::new(x, y)] = image_enhancement_algorithm
                    [input_image.get_surounding_points_num(&CordPoint::new(x, y))];
            }
        }
        input_image = new_image;
        input_image.default_point = if input_image.default_point == '.' {
            image_enhancement_algorithm[0]
        } else {
            image_enhancement_algorithm[1]
        }
    }

    input_image.map.into_iter().fold(
        0,
        |f_val, (_, n_val)| if n_val == '#' { f_val + 1 } else { f_val },
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    fn t1() {
        assert_eq_ansval!(35, day20_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day20_2("test"));
    }
}
