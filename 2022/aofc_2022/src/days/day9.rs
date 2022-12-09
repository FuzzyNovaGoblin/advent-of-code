use std::{collections::HashSet, fs};

#[derive(Debug)]
struct Rope {
    head_pos: (i32, i32),
    tail_pos: (i32, i32),
    tail_been_to: HashSet<(i32, i32)>,
}

impl Rope {

    fn move_right(&mut self) {
        self.head_pos.0 += 1;
        if self.get_too_far() {
            self.tail_pos.0 = self.head_pos.0 - 1;
            self.tail_pos.1 = self.head_pos.1;
        }
        self.tail_been_to.insert(self.tail_pos);
    }
    fn move_left(&mut self) {
        self.head_pos.0 -= 1;
        if self.get_too_far() {
            self.tail_pos.0 = self.head_pos.0 + 1;
            self.tail_pos.1 = self.head_pos.1;
        }
        self.tail_been_to.insert(self.tail_pos);
    }
    fn move_up(&mut self) {
        self.head_pos.1 += 1;
        if self.get_too_far() {
            self.tail_pos.0 = self.head_pos.0;
            self.tail_pos.1 = self.head_pos.1 - 1;
        }
        self.tail_been_to.insert(self.tail_pos);
    }
    fn move_down(&mut self) {
        self.head_pos.1 -= 1;
        if self.get_too_far() {
            self.tail_pos.0 = self.head_pos.0;
            self.tail_pos.1 = self.head_pos.1 + 1;
        }
        self.tail_been_to.insert(self.tail_pos);
    }

    fn get_too_far(&self) -> bool {
        if (self.head_pos.0 - self.tail_pos.0).abs() > 1 {
            true
        } else if (self.head_pos.1 - self.tail_pos.1).abs() > 1 {
            true
        } else {
            false
        }
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
        head_pos: (0, 0),
        tail_pos: (0, 0),
        tail_been_to: HashSet::new(),
    };

    data.lines().for_each(|line| {
        let line = line.split(' ').collect::<Vec<_>>();
        for _ in 0..line[1].parse::<u32>().unwrap() {
            match line[0] {
                "R" => rope.move_right(),
                "U" => rope.move_up(),
                "L" => rope.move_left(),
                "D" => rope.move_down(),
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
    let _data = fs::read_to_string(input_file);
    todo!()
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
