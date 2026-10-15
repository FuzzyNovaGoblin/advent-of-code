use md5::{Context, Digest};
use std::fs;

pub fn day4_1(file_name: &str) -> usize {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap().trim().to_owned();

    for i in 0 as usize.. {
        let mut hasher = Context::new();
        hasher.consume(format!("{data}{i}"));
        let hash = hasher.finalize();

        if hash[0..2] == [0; 2] && hash[2] >> 4 == 0 {
            return i;
        }

    }
    unreachable!()
}

pub fn day4_2(file_name: &str) -> usize {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap().trim().to_owned();

    for i in 0 as usize.. {
        let mut hasher = Context::new();
        hasher.consume(format!("{data}{i}"));
        let hash = hasher.finalize();

        if hash[0..3] == [0; 3] {
            return i;
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
        assert_eq_ansval!((), day4_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day4_2("test"));
    }
}
