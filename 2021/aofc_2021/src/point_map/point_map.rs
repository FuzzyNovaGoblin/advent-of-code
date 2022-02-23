
use std::{
    fmt::{Debug, Display},
    ops,
};



pub type CordPoint = (usize, usize);

#[derive(Default, Clone)]
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
    pub fn get_boardering_points_with_center(&self, point: CordPoint) -> Vec<CordPoint> {
        self.get_boardering_points(point, 0b_111_111_111)
    }

    pub fn get_boardering_points(&self, (x, y): CordPoint, include_mask: u16) -> Vec<CordPoint> {
        let mut ret_points = vec![];
        // first layer
        if y > 0 {
            if (0b_100_000_000 & include_mask) > 0 && x > 0 {
                ret_points.push((x - 1, y - 1));
            }
            if (0b_010_000_000 & include_mask) > 0 {
                ret_points.push((x, y - 1));
            }

            if (0b_001_000_000 & include_mask) > 0 && x < self.points.len() - 1 {
                ret_points.push((x + 1, y - 1));
            }
        }

        // second layer

        if (0b_000_100_000 & include_mask) > 0  && x > 0 {
            ret_points.push((x - 1, y));
        }

        if (0b_000_010_000 & include_mask) > 0 {
            ret_points.push((x, y));
        }

        if (0b_000_001_000 & include_mask) > 0 && x < self.points.len() - 1 {
            ret_points.push((x + 1, y));
        }

        // third layer
        if y < self.points[0].len() - 1 {
            if (0b_000_000_100 & include_mask) > 0 && x > 0 {
                ret_points.push((x - 1, y + 1));
            }
            if (0b_000_000_010 & include_mask) > 0 {
                ret_points.push((x, y + 1));
            }

            if (0b_000_000_001 & include_mask) > 0 && x < self.points.len() - 1 {
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

    pub fn len(&self) -> usize {
        let d = self.get_dimentions();
        d.0 * d.1
    }

    pub fn is_empty(&self) -> bool{
        self.len() == 0
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

auto trait NotString {}

impl !NotString for String {}

impl<T> From<PointMap<T>> for PointMap<String>
where
    T: Display + NotString,
{
    fn from(p: PointMap<T>) -> Self {
        PointMap::from(
            p.points
                .into_iter()
                .map(|v| v.into_iter().map(|item| format!("{}", item)).collect())
                .collect(),
        )
    }
}

impl<T> Debug for PointMap<T>
where
    T: Display + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PointMap {{\n\tdimentions: {:?},\n\tmap:\n{}\n}}",
            self.get_dimentions(),
            format!("{}", self)
                .split('\n')
                .map(|s| format!("\t     {}\n", s))
                .collect::<String>()
        )
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
                "*".repeat(self.points.len() * (max_size + 1) + 4)
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
    T: Display + Default + PartialOrd,
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
