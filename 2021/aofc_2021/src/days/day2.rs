use std::fs;

enum Directrion {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl Directrion {
    fn new(name: &str, num: i32) -> Directrion {
        if name == "forward" {
            Directrion::Forward(num)
        } else if name == "up" {
            Directrion::Up(num)
        } else {
            Directrion::Down(num)
        }
    }
}

pub fn day2_1() -> i32 {
    let input_file = format!("{}/aofc_2021/input/day2", env!("ADVENT_OF_CODE_2021"));
    let data = fs::read_to_string(input_file).unwrap();
    let data = data.split("\n").filter_map(|s| {
        let splt_data = s.split(" ").collect::<Vec<_>>();
        let udata = splt_data[1].parse::<i32>();
        match udata {
            Ok(v) => Some(Directrion::new(splt_data[0], v)),
            Err(_) => None,
        }
    });
    let mut d_pos = 0;
    let mut h_pos = 0;
    for d in data {
        match d {
            Directrion::Forward(v) => h_pos += v,
            Directrion::Up(v) => d_pos -= v,
            Directrion::Down(v) => d_pos += v,
        }
    }
    d_pos * h_pos
}

pub fn day2_2() -> i32{
    let input_file = format!("{}/aofc_2021/input/day2", env!("ADVENT_OF_CODE_2021"));
    let data = fs::read_to_string(input_file).unwrap();
    let data = data.split("\n").filter_map(|s| {
        let splt_data = s.split(" ").collect::<Vec<_>>();
        let udata = splt_data[1].parse::<i32>();
        match udata {
            Ok(v) => Some(Directrion::new(splt_data[0], v)),
            Err(_) => None,
        }
    });
    let mut d_pos = 0;
    let mut h_pos = 0;
    let mut aim = 0;
    for d in data {
        match d {
            Directrion::Forward(v) => {
                h_pos += v;
                d_pos += v*aim
            }
            Directrion::Up(v) => aim -= v,
            Directrion::Down(v) => aim += v,
        }
    }
    d_pos * h_pos
}
