use std::fs;

pub fn day25_1(file_name: &str) -> impl crate::AnsType {
let input_file = format!("{}/aofc_2021/input/{}", std::env::var("ADVENT_OF_CODE_2021").unwrap(), file_name);
    let _data = fs::read_to_string(input_file);
	todo!()
}

pub fn day25_2(file_name: &str) -> impl crate::AnsType {
let input_file = format!("{}/aofc_2021/input/{}", std::env::var("ADVENT_OF_CODE_2021").unwrap(), file_name);
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
        assert_eq_ansval!((), day25_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day25_2("test"));
    }
}
