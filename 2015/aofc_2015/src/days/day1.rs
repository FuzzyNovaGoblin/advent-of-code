use std::fs;

pub fn day1_1(file_name: &str) -> impl crate::AnsType{
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();
    data.chars().map(|c| if c == '(' { 1 } else { -1 }).sum::<i32>()
}

pub fn day1_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );

    let data = fs::read_to_string(input_file).unwrap();
    let mut sum = 0;
    for (i, c) in data.chars().enumerate(){
        sum += if c == '(' { 1 } else { -1 };
        if sum < 0 {
            return i + 1;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!((), day1_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day1_2("test"));
    }
}
