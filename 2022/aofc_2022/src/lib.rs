#![feature(default_free_fn, hash_drain_filter, auto_traits, negative_impls)]

use std::fmt::{Debug, Display};
pub mod days {
    pub mod day1;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
    pub mod day19;
    pub mod day2;
    pub mod day20;
    pub mod day21;
    pub mod day22;
    pub mod day23;
    pub mod day24;
    pub mod day25;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
    pub mod day7;
    pub mod day8;
    pub mod day9;
}

pub mod prelude {
    pub use super::days::day1::{day1_1, day1_2};
    pub use super::days::day10::{day10_1, day10_2};
    pub use super::days::day11::{day11_1, day11_2};
    pub use super::days::day12::{day12_1, day12_2};
    pub use super::days::day13::{day13_1, day13_2};
    pub use super::days::day14::{day14_1, day14_2};
    pub use super::days::day15::{day15_1, day15_2};
    pub use super::days::day16::{day16_1, day16_2};
    pub use super::days::day17::{day17_1, day17_2};
    pub use super::days::day18::{day18_1, day18_2};
    pub use super::days::day19::{day19_1, day19_2};
    pub use super::days::day2::{day2_1, day2_2};
    pub use super::days::day20::{day20_1, day20_2};
    pub use super::days::day21::{day21_1, day21_2};
    pub use super::days::day22::{day22_1, day22_2};
    pub use super::days::day23::{day23_1, day23_2};
    pub use super::days::day24::{day24_1, day24_2};
    pub use super::days::day25::{day25_1, day25_2};
    pub use super::days::day3::{day3_1, day3_2};
    pub use super::days::day4::{day4_1, day4_2};
    pub use super::days::day5::{day5_1, day5_2};
    pub use super::days::day6::{day6_1, day6_2};
    pub use super::days::day7::{day7_1, day7_2};
    pub use super::days::day8::{day8_1, day8_2};
    pub use super::days::day9::{day9_1, day9_2};
}

#[macro_export]
macro_rules! assert_eq_dbgfmt {
    ($a:expr, $b:expr) => {
        assert_eq!(
            format!("{:?}", $a),
            format!("{:?}", $b),
            "\nexpected {} to be {}",
            stringify!($b),
            stringify!($a)
        );
    };
}

#[macro_export]
macro_rules! assert_eq_ansval {
    ($a:expr, $b:expr) => {
        let a = $crate::AnsType::value(&$a);
        let b = $crate::AnsType::value(&$b);
        assert_eq!(a, b, "\nexpected {} to be {}", b, a);
    };
}

pub trait AnsType {
    fn value(&self) -> String;
}

auto trait AnsNotDisplay {}
impl !AnsNotDisplay for () {}
impl<T: Debug> !AnsNotDisplay for Option<T> {}

impl AnsType for () {
    fn value(&self) -> String {
        "()".into()
    }
}

impl<T: Debug> AnsType for Option<T> {
    fn value(&self) -> String {
        match self {
            Some(v) => format!("{:?}", v),
            None => String::from("None"),
        }
    }
}



impl<T: Display + AnsNotDisplay> AnsType for T {
    fn value(&self) -> String {
        format!("{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::days;
    #[test]
    fn day1_1() {
        assert_eq_ansval!(71502, days::day1::day1_1("day1"));
    }
    #[test]
    fn day1_2() {
        assert_eq_ansval!(208191, days::day1::day1_2("day1"));
    }
    #[test]
    #[ignore]
    fn day2_1() {
        assert_eq_ansval!((), days::day2::day2_1("day2"));
    }
    #[test]
    #[ignore]
    fn day2_2() {
        assert_eq_ansval!((), days::day2::day2_2("day2"));
    }
    #[test]
    #[ignore]
    fn day3_1() {
        assert_eq_ansval!((), days::day3::day3_1("day3"));
    }
    #[test]
    #[ignore]
    fn day3_2() {
        assert_eq_ansval!((), days::day3::day3_2("day3"));
    }
    #[test]
    #[ignore]
    fn day4_1() {
        assert_eq_ansval!((), days::day4::day4_1("day4"));
    }
    #[test]
    #[ignore]
    fn day4_2() {
        assert_eq_ansval!((), days::day4::day4_2("day4"));
    }
    #[test]
    #[ignore]
    fn day5_1() {
        assert_eq_ansval!((), days::day5::day5_1("day5"));
    }
    #[test]
    #[ignore]
    fn day5_2() {
        assert_eq_ansval!((), days::day5::day5_2("day5"));
    }
    #[test]
    #[ignore]
    fn day6_1() {
        assert_eq_ansval!((), days::day6::day6_1("day6"));
    }
    #[test]
    #[ignore]
    fn day6_2() {
        assert_eq_ansval!((), days::day6::day6_2("day6"));
    }
    #[test]
    #[ignore]
    fn day7_1() {
        assert_eq_ansval!((), days::day7::day7_1("day7"));
    }
    #[test]
    #[ignore]
    fn day7_2() {
        assert_eq_ansval!((), days::day7::day7_2("day7"));
    }
    #[test]
    #[ignore]
    fn day8_1() {
        assert_eq_ansval!((), days::day8::day8_1("day8"));
    }
    #[test]
    #[ignore]
    fn day8_2() {
        assert_eq_ansval!((), days::day8::day8_2("day8"));
    }
    #[test]
    #[ignore]
    fn day9_1() {
        assert_eq_ansval!((), days::day9::day9_1("day9"));
    }
    #[test]
    #[ignore]
    fn day9_2() {
        assert_eq_ansval!((), days::day9::day9_2("day9"));
    }
    #[test]
    #[ignore]
    fn day10_1() {
        assert_eq_ansval!((), days::day10::day10_1("day10"));
    }
    #[test]
    #[ignore]
    fn day10_2() {
        assert_eq_ansval!((), days::day10::day10_2("day10"));
    }
    #[test]
    #[ignore]
    fn day11_1() {
        assert_eq_ansval!((), days::day11::day11_1("day11"));
    }
    #[test]
    #[ignore]
    fn day11_2() {
        assert_eq_ansval!((), days::day11::day11_2("day11"));
    }
    #[test]
    #[ignore]
    fn day12_1() {
        assert_eq_ansval!((), days::day12::day12_1("day12"));
    }
    #[test]
    #[ignore]
    fn day12_2() {
        assert_eq_ansval!((), days::day12::day12_2("day12"));
    }
    #[test]
    #[ignore]
    fn day13_1() {
        assert_eq_ansval!((), days::day13::day13_1("day13"));
    }
    #[test]
    #[ignore]
    fn day13_2() {
        assert_eq_ansval!((), days::day13::day13_2("day13") );
    }
    #[test]
    #[ignore]
    fn day14_1() {
        assert_eq_ansval!((), days::day14::day14_1("day14"));
    }
    #[test]
    #[ignore]
    fn day14_2() {
        assert_eq_ansval!((), days::day14::day14_2("day14"));
    }
    #[test]
    #[ignore]
    fn day15_1() {
        assert_eq_ansval!((), days::day15::day15_1("day15"));
    }
    #[test]
    #[ignore]
    fn day15_2() {
        assert_eq_ansval!((), days::day15::day15_2("day15"));
    }
    #[test]
    #[ignore]
    fn day16_1() {
        assert_eq_ansval!((), days::day16::day16_1("day16"));
    }
    #[test]
    #[ignore]
    fn day16_2() {
        assert_eq_ansval!((), days::day16::day16_2("day16"));
    }
    #[test]
    #[ignore]
    fn day17_1() {
        assert_eq_ansval!((), days::day17::day17_1("day17"));
    }
    #[test]
    #[ignore]
    fn day17_2() {
        assert_eq_ansval!((), days::day17::day17_2("day17"));
    }
    #[test]
    #[ignore]
    fn day18_1() {
        assert_eq_ansval!((), days::day18::day18_1("day18"));
    }
    #[test]
    #[ignore]
    fn day18_2() {
        assert_eq_ansval!((), days::day18::day18_2("day18"));
    }
    #[test]
    #[ignore]
    fn day19_1() {
        assert_eq_ansval!((), days::day19::day19_1("day19"));
    }
    #[test]
    #[ignore]
    fn day19_2() {
        assert_eq_ansval!((), days::day19::day19_2("day19"));
    }
    #[test]
    #[ignore]
    fn day20_1() {
        assert_eq_ansval!((), days::day20::day20_1("day20"));
    }
    #[test]
    #[ignore]
    fn day20_2() {
        assert_eq_ansval!((), days::day20::day20_2("day20"));
    }
    #[test]
    #[ignore]
    fn day21_1() {
        assert_eq_ansval!((), days::day21::day21_1("day21"));
    }
    #[test]
    #[ignore]
    fn day21_2() {
        assert_eq_ansval!((), days::day21::day21_2("day21"));
    }
    #[test]
    #[ignore]
    fn day22_1() {
        assert_eq_ansval!((), days::day22::day22_1("day22"));
    }
    #[test]
    #[ignore]
    fn day22_2() {
        assert_eq_ansval!((), days::day22::day22_2("day22"));
    }
    #[test]
    #[ignore]
    fn day23_1() {
        assert_eq_ansval!((), days::day23::day23_1("day23"));
    }
    #[test]
    #[ignore]
    fn day23_2() {
        assert_eq_ansval!((), days::day23::day23_2("day23"));
    }
    #[test]
    #[ignore]
    fn day24_1() {
        assert_eq_ansval!((), days::day24::day24_1("day24"));
    }
    #[test]
    #[ignore]
    fn day24_2() {
        assert_eq_ansval!((), days::day24::day24_2("day24"));
    }
    #[test]
    #[ignore]
    fn day25_1() {
        assert_eq_ansval!((), days::day25::day25_1("day25"));
    }
    #[test]
    #[ignore]
    fn day25_2() {
        assert_eq_ansval!((), days::day25::day25_2("day25"));
    }
}
