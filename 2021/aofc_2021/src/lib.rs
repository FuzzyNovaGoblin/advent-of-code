#![feature(hash_drain_filter)]
#![feature(default_free_fn)]
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
pub mod point_map;

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
        assert_eq!(format!("{:?}", $a), format!("{:?}", $b), "\nexpected {} to be {}", stringify!($b), stringify!($a));
    };
}