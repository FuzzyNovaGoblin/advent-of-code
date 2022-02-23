#![feature(concat_idents)]

use std::env::args;

use aofc_2021::{prelude::*, AnsType};

macro_rules! getday_fn {
    ( $day_num:expr , $($n:ident),*) => {
        match format!("y{}",$day_num).as_str() {
            $(
                stringify!($n) => {
                    println!(
                        "part 1: {}",
                        concat_idents!(da, $n, _1)(format!("day{}", $day_num).as_str()).value()
                    );
                    println!(
                        "part 2: {}",
                        concat_idents!(da, $n, _2)(format!("day{}", $day_num).as_str()).value()
                    );
                }
            )*
            _ => unreachable!()
        }
    };
}

fn main() {
    let day_num = args()
        .into_iter().nth(1)
        .expect("pass a day value")
        .parse::<u8>()
        .expect("not a valid integer");

    getday_fn!(
        day_num, y1, y2, y3, y4, y5, y6, y7, y8, y9, y10, y11, y12, y13, y14, y15, y16, y17, y18,
        y19, y20, y21, y22, y23, y24, y25
    );

    // println!( "part 1: {}",day15_1("test").value());
}
