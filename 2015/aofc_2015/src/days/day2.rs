use std::{cmp::min, fs};

#[derive(Debug)]
struct BoxDimensions {
    l: i32,
    w: i32,
    h: i32,
}

pub fn day2_1(file_name: &str) -> i32 {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();

    data.lines()
        .filter_map(|line| {
            if line.len() >= 5 {
                Some(BoxDimensions::from_line(line))
            } else {
                None
            }
        })
        .map(|d| d.wrapping_needed())
        .sum()
}

pub fn day2_2(file_name: &str) -> i32 {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();

    data.lines()
        .filter_map(|line| {
            if line.len() >= 5 {
                Some(BoxDimensions::from_line(line))
            } else {
                None
            }
        })
        .map(|d| d.ribbon_needed())
        .sum()
}

impl BoxDimensions {
    fn from_line<S: AsRef<str>>(line: S) -> BoxDimensions {
        let parts = line
            .as_ref()
            .split('x')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        BoxDimensions {
            l: parts[0],
            w: parts[1],
            h: parts[2],
        }
    }

    fn volume(&self) -> i32 {
        self.l * self.w * self.h
    }

    fn ribbon_needed(&self) -> i32 {
        min(
            min((self.l + self.w) * 2, (self.l + self.h) * 2),
            (self.h + self.w) * 2,
        ) + self.volume()
    }

    fn smallest_side_area(&self) -> i32 {
        min(min(self.l * self.w, self.l * self.h), self.h * self.w)
    }

    fn surface_area(&self) -> i32 {
        (self.l * self.w + self.l * self.h + self.h * self.w) * 2
    }

    fn wrapping_needed(&self) -> i32 {
        self.surface_area() + self.smallest_side_area()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!((), day2_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day2_2("test"));
    }
}
