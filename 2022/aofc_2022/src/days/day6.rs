use std::{collections::HashSet, fs};

pub fn day6_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file);

    for (i, window) in data
        .unwrap()
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .enumerate()
    {
        if window[0] != window[1]
            && window[0] != window[2]
            && window[0] != window[3]
            && window[1] != window[2]
            && window[1] != window[3]
            && window[2] != window[3]
        {
            return i + 4;
        }
    }
    panic!()
}

pub fn day6_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file);

    for (i, window) in data
        .unwrap()
        .chars()
        .collect::<Vec<_>>()
        .windows(14)
        .enumerate()
    {
        let mut working_set = HashSet::new();
        let mut failed = false;

        for c in window {
            if working_set.contains(c) {
                failed = true;
                break;
            } else {
                working_set.insert(c);
            }
        }
        if !failed {
            return i + 14;
        }
    }
    panic!()
}
