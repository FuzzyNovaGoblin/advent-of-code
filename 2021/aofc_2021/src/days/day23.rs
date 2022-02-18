use std::fs;

pub fn day23_1(file_name: &str) -> impl std::fmt::Debug {
let input_file = format!("{}/aofc_2021/input/{}", env!("ADVENT_OF_CODE_2021"), file_name);
    let _data = fs::read_to_string(input_file);
	todo!()
}

pub fn day23_2(file_name: &str) -> impl std::fmt::Debug {
let input_file = format!("{}/aofc_2021/input/{}", env!("ADVENT_OF_CODE_2021"), file_name);
    let _data = fs::read_to_string(input_file);
	todo!()
}



#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_dbgfmt;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_dbgfmt!((), day23_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_dbgfmt!((), day23_2("test"));
    }
}
