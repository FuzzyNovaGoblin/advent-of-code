#![feature(concat_idents)]

use std::{collections::HashSet, env::args};

use aofc_2021::{prelude::*, AnsType};

macro_rules! getday_fn {
    ( $day_num:expr, $do_run:expr, $is_test:expr , $($n:ident),*) => {
        match format!("y{}",$day_num).as_str() {
            $(
                stringify!($n) => {
                    if $do_run.0{
                        println!(
                            "part 1: {}",
                            if $is_test{
                                concat_idents!(da, $n, _1)("test").value()
                            }
                            else{
                                concat_idents!(da, $n, _1)(format!("day{}", $day_num).as_str()).value()
                            }
                        );
                    }

                    if $do_run.1{
                        println!(
                            "part 2: {}",
                             if $is_test{
                                concat_idents!(da, $n, _2)("test").value()
                            }
                            else{
                                concat_idents!(da, $n, _2)(format!("day{}", $day_num).as_str()).value()
                            }
                        );
                    }
                }
            )*
            _ => unreachable!()
        }
    };
}

fn main() {
    let mut args = args();
    let day_num = args
        .nth(1)
        .expect("pass a day value")
        .parse::<u8>()
        .expect("not a valid integer");
    let args = args.collect::<HashSet<_>>();
    let test = args.contains("test");
    let do_run = (!args.contains("-1"), !args.contains("-2"));

    getday_fn!(
        day_num, do_run, test, y1, y2, y3, y4, y5, y6, y7, y8, y9, y10, y11, y12, y13, y14, y15,
        y16, y17, y18, y19, y20, y21, y22, y23, y24, y25
    );
}
