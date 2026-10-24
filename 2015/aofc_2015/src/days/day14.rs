use std::{cmp::min, fs};

#[derive(Debug)]
struct Deer {
    #[allow(unused)]
    name: String,
    speed: i32,
    fly_time: i32,
    rest_time: i32,
}

impl Deer {
    fn from_line<S: AsRef<str>>(line: S) -> Deer {
        let parts = line.as_ref().split(' ').collect::<Vec<_>>();
        Deer {
            speed: parts[3].parse().unwrap(),
            name: parts[0].to_owned(),
            fly_time: parts[6].parse().unwrap(),
            rest_time: parts[13].parse().unwrap(),
        }
    }
    fn cycle_time(&self) -> i32 {
        self.fly_time + self.rest_time
    }
    fn fly_per_cycle(&self) -> i32 {
        self.fly_time * self.speed
    }

    fn flight_per_time(&self, time: i32) -> i32 {
        let cycles = time / self.cycle_time();
        let remainder = time % self.cycle_time();

        (cycles * self.fly_per_cycle()) + (min(self.fly_time, remainder) * self.speed)
    }
}

fn largest_index(values: Vec<i32>) -> usize {
    let mut largest = (0, 0);
    for (i, v) in values.iter().enumerate() {
        if *v > largest.1 {
            largest = (i, *v);
        }
    }
    largest.0
}

pub fn day14_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|l| Deer::from_line(l))
        .map(|d| d.flight_per_time(2503))
        .max()
        .unwrap()
}

pub fn day14_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let deer = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|l| Deer::from_line(l))
        .collect::<Vec<_>>();

    let mut points = vec![0; deer.len()];

    for second in 1..=2503 {
        let dis = deer
            .iter()
            .map(|d| d.flight_per_time(second))
            .collect::<Vec<_>>();
        points[largest_index(dis)] += 1;
    }
    points.into_iter().max()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!((), day14_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day14_2("test"));
    }
}
