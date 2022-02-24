use std::{cell::RefCell, fs, rc::Rc};

use crate::point_map::{CordPointTuple, DimentionIter, PointMap};

impl PointMap<u32> {}

fn handle_explosion(
    point_map: &mut PointMap<u32>,
    start_point: CordPointTuple,
    already_exploded: Rc<RefCell<Vec<CordPointTuple>>>,
    flashes: Rc<RefCell<u128>>,
) {
    {
        let mut b = already_exploded.borrow_mut();
        if b.contains(&start_point) {
            return;
        }
        let mut fla = flashes.borrow_mut();
        *fla += 1;
        b.push(start_point)
    }

    for point in point_map.get_boardering_points_with_center(start_point) {
        point_map[point] += 1;
    }
}

pub fn day11_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let _data = fs::read_to_string(input_file);
    let mut point_map = PointMap::default();
    for (y, line) in _data.unwrap().split('\n').enumerate() {
        for (x, char) in line.chars().enumerate() {
            point_map[(x, y)] = char.to_digit(10).unwrap();
        }
    }
    let dimentions = point_map.get_dimentions();
    let flashes = Rc::new(RefCell::new(0));
    for _ in 0..100 {
        let already_exploded = Rc::new(RefCell::new(vec![]));
        for point in DimentionIter::new(dimentions) {
            point_map[point] += 1;
        }
        let mut trip = true;
        while trip {
            trip = false;
            for point in DimentionIter::new(dimentions) {
                if point_map[point] > 9 {
                    {
                        let b = already_exploded.borrow();
                        if b.contains(&point) {
                            continue;
                        }
                    }
                    trip = true;
                    handle_explosion(
                        &mut point_map,
                        point,
                        already_exploded.clone(),
                        flashes.clone(),
                    );
                }
            }
        }
        for point in DimentionIter::new(dimentions) {
            if point_map[point] > 9 {
                point_map[point] = 0
            }
        }
    }

    flashes.take()
}

pub fn day11_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let _data = fs::read_to_string(input_file);
    let mut point_map = PointMap::default();
    for (y, line) in _data.unwrap().split('\n').enumerate() {
        for (x, char) in line.chars().enumerate() {
            point_map[(x, y)] = char.to_digit(10).unwrap();
        }
    }
    let dimentions = point_map.get_dimentions();
    let map_len = dimentions.0 * dimentions.1;
    let flashes = Rc::new(RefCell::new(0));
    let already_exploded = Rc::new(RefCell::new(vec![]));
    let mut count = 0;

    while already_exploded.borrow().len() < map_len {
        {
            let mut b = already_exploded.borrow_mut();
            b.clear();
        }
        count += 1;

        for point in DimentionIter::new(dimentions) {
            point_map[point] += 1;
        }
        let mut trip = true;
        while trip {
            trip = false;
            for point in DimentionIter::new(dimentions) {
                if point_map[point] > 9 {
                    {
                        let b = already_exploded.borrow();
                        if b.contains(&point) {
                            continue;
                        }
                    }
                    trip = true;
                    handle_explosion(
                        &mut point_map,
                        point,
                        already_exploded.clone(),
                        flashes.clone(),
                    );
                }
            }
        }
        for point in DimentionIter::new(dimentions) {
            if point_map[point] > 9 {
                point_map[point] = 0
            }
        }
    }

    count
}
