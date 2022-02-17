#![allow(dead_code)]
use std::{
    fmt::{Debug, Display},
    ops,
};

use crate::point_map::DimentionIter;


pub type CordPoint = (usize, usize);

#[derive(Default)]
pub struct PointMap<T> {
    points: Vec<Vec<T>>,
}

impl<T> PointMap<T>
where
    T: Display + Default,
{
    pub fn from(points: Vec<Vec<T>>) -> Self {
        Self { points }
    }

    pub fn get_boardering_points(&self, (x, y): CordPoint) -> Vec<CordPoint> {
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
            ret_points.push((x - 1, y));
        }
        ret_points.push((x, y));

        if x < self.points.len() - 1 {
            ret_points.push((x + 1, y));
        }

        // third layer
        if y < self.points[0].len() - 1 {
            if x > 0 {
                ret_points.push((x - 1, y + 1));
            }
            ret_points.push((x, y + 1));

            if x < self.points.len() - 1 {
                ret_points.push((x + 1, y + 1));
            }
        }

        ret_points
    }

    pub fn print_around_point(&self, x: usize, y: usize) {
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

impl<T> PointMap<T> {
    pub fn get_dimentions(&self) -> CordPoint {
        (
            self.points.len(),
            self.points.get(0).unwrap_or(&Vec::new()).len(),
        )
    }
}

impl<T> Debug for PointMap<T>
where
    T: Display + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PointMap {{\n\tdimentions: {:?},\n\tmap:\n{}\n}}", self.get_dimentions(), format!("{}", self).split("\n").map(|s|format!("\t     {}\n", s)).collect::<String>())
    }


}

impl<T> Display for PointMap<T>
where
    T: Display + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut max_size = 0;
        for point in DimentionIter::new(self.get_dimentions()) {
            max_size = std::cmp::max(format!("{}", self[point]).len(), max_size);
        }

        let write_h_line = |f: &mut std::fmt::Formatter<'_>| -> std::fmt::Result {
            write!(
                f,
                "{}",
                std::iter::repeat("*")
                    .take(self.points.len() * (max_size + 1) + 4)
                    .collect::<String>()
            )
        };

        write_h_line(f)?;
        writeln!(f)?;
        for y in 0..self.points[0].len() {
            write!(f, "**")?;
            for x in 0..self.points.len() {
                write!(f, "{:max_size$} ", self.points[x][y], max_size = max_size)?;
            }
            writeln!(f, "**")?;
        }
        write_h_line(f)?;

        Ok(())
    }
}

impl<T> ops::Index<CordPoint> for PointMap<T> {
    type Output = T;

    fn index(&self, (x, y): CordPoint) -> &Self::Output {
        &self.points[x][y]
    }
}

impl<T> ops::IndexMut<CordPoint> for PointMap<T>
where
    T: Display + Default + PartialEq<u32> + PartialOrd,
{
    fn index_mut(&mut self, (x, y): CordPoint) -> &mut Self::Output {
        let (_, height) = self.get_dimentions();
        while x >= self.points.len() {
            self.points.push(Default::default());
            self.points.last_mut().into_iter().for_each(|column| {
                while column.len() < height {
                    column.push(Default::default());
                }
            });
        }
        while y >= self.points[x].len() {
            self.points
                .iter_mut()
                .for_each(|column| column.push(Default::default()));
        }

        &mut self.points[x][y]
    }
}
