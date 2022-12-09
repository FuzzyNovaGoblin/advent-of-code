use std::{collections::HashSet, fmt::Display, fs};

#[derive(Debug)]
struct Rope {
    knots: Vec<(i32, i32)>,
    tail_been_to: HashSet<(i32, i32)>,
}

impl Rope {
    fn move_dir(&mut self, direction: (i32, i32), head_index: usize, tail_index: usize) {
        self.knots[head_index].0 += direction.0;
        self.knots[head_index].1 += direction.1;

        if tail_index >= self.knots.len() {
            self.tail_been_to.insert(self.knots[head_index]);
            return;
        }

        self.move_dir(
            self.distance(head_index, tail_index),
            tail_index,
            tail_index + 1,
        );
    }

    fn distance(&self, head_index: usize, tail_index: usize) -> (i32, i32) {
        let head = self.knots[head_index];
        let tail = self.knots[tail_index];
        let distance = ((head.0 - tail.0), (head.1 - tail.1));

        if !(distance.0.abs() > 1 || distance.1.abs() > 1) {
            return (0, 0);
        }

        if distance.0 == 0 {
            return (0, distance.1 / 2);
        }
        if distance.1 == 0 {
            return (distance.0 / 2, 0);
        }

        (
            if distance.0.is_negative() { -1 } else { 1 },
            if distance.1.is_negative() { -1 } else { 1 },
        )
    }
}

pub fn day9_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();

    let mut rope = Rope {
        knots: vec![(0, 0); 2],
        tail_been_to: HashSet::new(),
    };

    data.lines().for_each(|line| {
        let line = line.split(' ').collect::<Vec<_>>();
        for _ in 0..line[1].parse::<u32>().unwrap() {
            match line[0] {
                "R" => rope.move_dir((1, 0), 0, 1),
                "U" => rope.move_dir((0, 1), 0, 1),
                "L" => rope.move_dir((-1, 0), 0, 1),
                "D" => rope.move_dir((0, -1), 0, 1),
                _ => unreachable!(),
            }
        }
    });

    rope.tail_been_to.len()
}

pub fn day9_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();

    let mut rope = Rope {
        knots: vec![(0, 0); 10],
        tail_been_to: HashSet::new(),
    };

    data.lines().for_each(|line| {
        let line = line.split(' ').collect::<Vec<_>>();
        for _ in 0..line[1].parse::<u32>().unwrap() {
            match line[0] {
                "R" => rope.move_dir((1, 0), 0, 1),
                "U" => rope.move_dir((0, 1), 0, 1),
                "L" => rope.move_dir((-1, 0), 0, 1),
                "D" => rope.move_dir((0, -1), 0, 1),
                _ => unreachable!(),
            }
        }
    });

    rope.tail_been_to.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!((), day9_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day9_2("test"));
    }
}
