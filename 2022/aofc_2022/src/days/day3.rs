use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn letter_to_number(c: char) -> u32 {
    if ('a'..='z').contains(&c) {
        (c as u8 as u32) - b'a' as u32 + 1
    } else {
        (c as u8 as u32) - b'A' as u32 + 26 + 1
    }
}

pub fn day3_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file);
    data.unwrap()
        .lines()
        .map(|line| {
            let s = line.len() / 2;
            (&line[0..s], &line[s..])
        })
        .map(|(comp1, comp2)| {
            for c1 in comp1.chars() {
                for c2 in comp2.chars() {
                    if c1 == c2 {
                        return letter_to_number(c1);
                    }
                }
            }
            panic!()
        })
        .sum::<u32>()
}

pub fn day3_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();

    data.lines()
        .step_by(3)
        .zip(data.lines().skip(1).step_by(3))
        .zip(data.lines().skip(2).step_by(3))
        .map(|group| {
            let group = vec![group.0.0, group.0.1, group.1];
            let mut working_hash_map: HashMap<char, u32> = HashMap::new();

            for line in group {
                let mut working_hash_set = HashSet::new();
                for c in line.chars() {
                    working_hash_set.insert(c);
                }
                for c in working_hash_set {
                    *working_hash_map.entry(c).or_default() += 1;
                    if *working_hash_map.entry(c).or_default() == 3 {
                        return letter_to_number(c);
                    }
                }
            }
            panic!()
        })
        .sum::<u32>()
}
