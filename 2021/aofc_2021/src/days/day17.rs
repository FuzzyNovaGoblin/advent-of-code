use std::fs;

#[derive(Debug, Clone)]
struct TargetArea {
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
}

impl TargetArea {
    fn new(x_min: usize, x_max: usize, y_min: usize, y_max: usize) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
}

pub fn day17_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let data = fs::read_to_string(input_file)
        .unwrap()
        .chars()
        .skip(12)
        .collect::<String>()
        .split(',')
        .map(|s| {
            s.chars()
                .skip(3)
                .to_owned()
                .collect::<String>()
                .split("..")
                .map(|num_str| i32::from_str_radix(num_str, 10).expect("num from string"))
                .collect()
        })
        .collect::<Vec<Vec<_>>>();
    dbg!(data);
    todo!()
}

pub fn day17_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
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
    fn t1() {
        assert_eq_ansval!(45, day17_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day17_2("test"));
    }
}
