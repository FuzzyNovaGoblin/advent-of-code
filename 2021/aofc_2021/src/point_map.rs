use std::{collections::HashSet, fmt::Display, ops};

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
            std::iter::repeat("*")
                .take(self.points.len() + 4)
                .collect::<String>()
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
            std::iter::repeat("*")
                .take(self.points.len() + 4)
                .collect::<String>()
        );
    }

    fn get_around_point(&mut self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut ret_points = vec![];
        // first layer
        if y > 0 {
            if x > 0 {
                ret_points.push((x - 1, y - 1));
            }
            ret_points.push((x, y - 1));

            if x < self.points.len() - 1 {
                ret_points.push((x + 1, y - 1));
            }
        }

        // second layer

        if x > 0 {
            ret_points.push((x, y));
        }
        ret_points.push((x, y));

        if x < self.points.len() - 1 {
            ret_points.push((x, y));
        }

        // third layer
        if y < self.points[0].len() - 1 {
            if x > 0 {
                ret_points.push((x, y));
            }
            ret_points.push((x, y));

            if x < self.points.len() - 1 {
                ret_points.push((x, y));
            }
        }

        ret_points
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
