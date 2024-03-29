use std::fs;

pub fn day20_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let _data = fs::read_to_string(input_file);
    todo!()
}

pub fn day20_2(file_name: &str) -> impl crate::AnsType {
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
        assert_eq_ansval!((), day20_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day20_2("test"));
    }
}
