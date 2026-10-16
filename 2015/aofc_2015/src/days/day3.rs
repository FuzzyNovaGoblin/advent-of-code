use std::{collections::HashSet, fs};

fn dir_to_vector(dir: char) -> (i32, i32) {
    match dir {
        '^' => (0, 1),
        'v' => (0, -1),
        '>' => (1, 0),
        '<' => (-1, 0),
        _ => (0, 0),
    }
}

pub fn day3_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let mut seen_house = HashSet::new();
    let mut chords = (0, 0);
    seen_house.insert(chords);
    // let data =
    for v in fs::read_to_string(input_file)
        .unwrap()
        .chars()
        .map(dir_to_vector)
    {
        chords = (chords.0+v.0, chords.1+v.1);
        seen_house.insert(chords);
    }
    seen_house.len()
}

pub fn day3_2(file_name: &str) -> impl crate::AnsType {
     let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let mut seen_house = HashSet::new();
    let mut chords = [(0, 0),(0, 0)];
    seen_house.insert(chords[0]);
    // let data =
    for (i, v) in fs::read_to_string(input_file)
        .unwrap()
        .chars()
        .map(dir_to_vector).enumerate()
    {
        chords[i%2] = (chords[i%2].0+v.0, chords[i%2].1+v.1);
        seen_house.insert(chords[i%2]);
    }
    seen_house.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!((), day3_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day3_2("test"));
    }
}
