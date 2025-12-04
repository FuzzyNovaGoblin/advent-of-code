use std::fs;

#[derive(Debug)]
struct IdRange {
    start: u64,
    end: u64,
}

impl IdRange {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
    fn find_repeats(&self) -> Vec<u64> {
        let mut repeat_ids = Vec::new();
        for i in self.start..=self.end {
            let num_string = format!("{}", i);
            let num = num_string.chars().collect::<Vec<_>>();
            for j in 1..=num.len() / 2 {
                if num.len() % j != 0 {
                    continue;
                }
                let test_val = std::iter::repeat(String::from_iter(&num[0..j]))
                    .take(num.len() / j)
                    .collect::<String>();
                if test_val == num_string {
                    repeat_ids.push(i);
                    break;
                }
            }
        }
        repeat_ids
    }
    fn find_doubles(&self) -> Vec<u64> {
        let mut repeat_ids = Vec::new();
        for i in self.start..=self.end {
            let num_string = format!("{}", i);
            let num = num_string.chars().collect::<Vec<_>>();

            if num.len() % 2 != 0 {
                continue;
            }

            let h = num.len() / 2;
            let test_val = std::iter::repeat(String::from_iter(&num[0..h]))
                .take(num.len() / h)
                .collect::<String>();
            if test_val == num_string {
                repeat_ids.push(i);
            }
        }
        repeat_ids
    }
}

pub fn day2_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2025/input/{}",
        env!("ADVENT_OF_CODE_2025"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();

    let data = data.trim().split(',').map(|s| {
        let parts = s.split('-').collect::<Vec<_>>();
        IdRange::new(parts[0].parse().unwrap(), parts[1].parse().unwrap())
    });

    data.map(|r| r.find_doubles()).flatten().sum::<u64>()
}

pub fn day2_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2025/input/{}",
        env!("ADVENT_OF_CODE_2025"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();
    // let data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    let data = data.trim().split(',').map(|s| {
        let parts = s.split('-').collect::<Vec<_>>();
        IdRange::new(parts[0].parse().unwrap(), parts[1].parse().unwrap())
    });

    // dbg!(data.clone().map(|r| r.find_repeats()).flatten().collect::<Vec<_>>());
    data.map(|r| r.find_repeats()).flatten().sum::<u64>()
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
