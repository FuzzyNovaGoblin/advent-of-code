use std::{cmp, fs, ops};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct FreshRange {
    start: u64,
    end: u64,
}
#[derive(Debug)]
struct RangeSet {
    ranges: Vec<FreshRange>,
}

impl RangeSet {
    fn new<S: AsRef<str>>(s: S) -> Self {
        Self {
            ranges: s.as_ref().lines().map(FreshRange::new).collect(),
        }
    }
    fn in_range(&self, num: u64) -> bool {
        for r in self.ranges.iter() {
            if r.in_range(num) {
                // dbg!((&num, &r));
                return true;
            };
        }
        return false;
    }

    fn combine_overlaps(&mut self) {
        let mut new_ranges = Vec::new();
        let mut made_change;
        loop {
            made_change = false;
            for r in &self.ranges {
                match find_overlapping_range(&new_ranges, r) {
                    Some(i) => {
                        new_ranges[i] += r;
                        made_change = true
                    }
                    None => new_ranges.push(r.clone()),
                }
            }
            self.ranges = new_ranges;
            self.ranges.sort();
            new_ranges = Vec::new();
            if !made_change {
                break;
            }
        }
    }

    fn get_possible_count(&mut self) -> u64 {
        self.combine_overlaps();
        self.ranges.iter().map(|r| r.get_possible_count()).sum()
    }
}

fn find_overlapping_range(ranges: &Vec<FreshRange>, checker: &FreshRange) -> Option<usize> {
    for (i, r) in ranges.iter().enumerate() {
        if r.does_overlap(checker) {
            return Some(i);
        }
    }
    None
}

impl ops::AddAssign<&FreshRange> for FreshRange {
    fn add_assign(&mut self, rhs: &FreshRange) {
        *self = &*self + rhs
    }
}

impl ops::Add<&FreshRange> for &FreshRange {
    type Output = FreshRange;

    fn add(self, rhs: &FreshRange) -> Self::Output {
        FreshRange {
            start: cmp::min(self.start, rhs.start),
            end: cmp::max(self.end, rhs.end),
        }
    }
}

impl FreshRange {
    fn new<S: AsRef<str>>(s: S) -> Self {
        let l = s.as_ref().split('-').collect::<Vec<_>>();
        Self {
            start: l[0].parse::<u64>().unwrap(),
            end: l[1].parse::<u64>().unwrap(),
        }
    }

    // fn combine_ranges(&self, rhs: &FreshRange) -> FreshRange {
    //     FreshRange {
    //         start: cmp::min(self.start, rhs.start),
    //         end: cmp::max(self.end, rhs.end),
    //     }
    // }

    fn does_overlap(&self, rhs: &Self) -> bool {
        rhs.start >= self.start && rhs.start <= self.end
            || rhs.end >= self.start && rhs.end <= self.end
    }

    fn get_possible_count(&self) -> u64 {
        (self.end + 1) - self.start
    }

    fn in_range(&self, num: u64) -> bool {
        num >= self.start && num <= self.end
    }
}

pub fn day5_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2025/input/{}",
        env!("ADVENT_OF_CODE_2025"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();

    let data = data.split("\n\n").collect::<Vec<_>>();
    let ranges = RangeSet::new(data[0]);
    data[1]
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .filter(|n| ranges.in_range(*n))
        .count()
}

pub fn day5_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2025/input/{}",
        env!("ADVENT_OF_CODE_2025"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();

    let data = data.split("\n\n").collect::<Vec<_>>();
    let mut ranges = RangeSet::new(data[0]);

    ranges.get_possible_count()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!("758", day5_1("day5"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!("343143696885053", day5_2("day5"));
    }
}
