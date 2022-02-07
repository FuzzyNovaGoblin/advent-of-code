use std::fs;

fn cost_to_align(pos: i64, list: &Vec<i64>) -> i64 {
    let mut cost = 0;
    for p in list {
        cost+=(p-pos).abs()
    }
    cost
}

fn cost_to_alignv2(pos: i64, list: &Vec<i64>) -> i64 {
    let mut cost = 0;
    for p in list {
        let v =(p-pos).abs();

        cost+= (v*(v+1))/2
    }
    cost
}

pub fn day7_1() -> i64 {
    let input_file = format!("{}/aofc_2021/input/day7", env!("ADVENT_OF_CODE_2021"));
    let _data = fs::read_to_string(input_file);
    let data: Vec<_> = _data
        .unwrap()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let max_pos = data.iter().max().unwrap();

    let mut min_cost = cost_to_align(0, &data);
    for pos in 1..*max_pos {
        min_cost = std::cmp::min(min_cost, cost_to_align(pos, &data));
    }
    min_cost
}

pub fn day7_2() -> i64{
    let input_file = format!("{}/aofc_2021/input/day7", env!("ADVENT_OF_CODE_2021"));
    let _data = fs::read_to_string(input_file);

      let data: Vec<_> = _data
        .unwrap()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let max_pos = data.iter().max().unwrap();

    let mut min_cost = cost_to_alignv2(0, &data);
    for pos in 1..*max_pos {
        min_cost = std::cmp::min(min_cost, cost_to_alignv2(pos, &data));
    }
    min_cost
}
